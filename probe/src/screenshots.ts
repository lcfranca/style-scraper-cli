import { mkdir } from "node:fs/promises";
import type { Page } from "playwright";
import type { CaptureRequest } from "./schema";

type ScreenshotFact = {
  id: string;
  type: "viewport" | "full-page" | "element";
  path?: string;
  sha256: string;
  width: number;
  height: number;
  device_scale_factor: number;
  viewport: CaptureRequest["viewport"];
  target_node_id?: string;
};

export async function captureScreenshots(
  page: Page,
  request: CaptureRequest
): Promise<ScreenshotFact[]> {
  if (!request.includeScreenshots || request.redactImages) {
    return [];
  }

  const outputDir = request.screenshotDir?.replace(/\/$/, "");
  if (outputDir) {
    await mkdir(outputDir, { recursive: true });
  }

  const screenshots: ScreenshotFact[] = [];
  screenshots.push(await captureViewport(page, request, outputDir));
  if (request.screenshot === "full-page") {
    screenshots.push(await captureFullPage(page, request, outputDir));
  }
  if (request.screenshot === "elements") {
    screenshots.push(...await captureElementScreenshots(page, request, outputDir));
  }
  return screenshots;
}

async function captureViewport(
  page: Page,
  request: CaptureRequest,
  outputDir?: string
): Promise<ScreenshotFact> {
  const bytes = await page.screenshot({ fullPage: false });
  const path = outputDir ? `${outputDir}/viewport-${viewportSlug(request)}.png` : undefined;
  if (path) {
    await Bun.write(path, bytes);
  }
  return {
    id: `screenshot:viewport:${viewportSlug(request)}`,
    type: "viewport",
    path,
    sha256: await hashBytes(bytes),
    width: request.viewport.width,
    height: request.viewport.height,
    device_scale_factor: request.viewport.device_scale_factor,
    viewport: request.viewport
  };
}

async function captureFullPage(
  page: Page,
  request: CaptureRequest,
  outputDir?: string
): Promise<ScreenshotFact> {
  const size = await page.evaluate(() => ({
    width: Math.ceil(Math.max(document.documentElement.scrollWidth, document.body?.scrollWidth ?? 0)),
    height: Math.ceil(Math.max(document.documentElement.scrollHeight, document.body?.scrollHeight ?? 0))
  }));
  const bytes = await page.screenshot({ fullPage: true });
  const path = outputDir ? `${outputDir}/full-page-${viewportSlug(request)}.png` : undefined;
  if (path) {
    await Bun.write(path, bytes);
  }
  return {
    id: `screenshot:full-page:${viewportSlug(request)}`,
    type: "full-page",
    path,
    sha256: await hashBytes(bytes),
    width: size.width,
    height: size.height,
    device_scale_factor: request.viewport.device_scale_factor,
    viewport: request.viewport
  };
}

async function captureElementScreenshots(
  page: Page,
  request: CaptureRequest,
  outputDir?: string
): Promise<ScreenshotFact[]> {
  const targets = await page.evaluate(() => {
    const elements = Array.from(document.querySelectorAll("*"));
    return elements
      .map((element, index) => {
        const rect = element.getBoundingClientRect();
        const style = getComputedStyle(element);
        const tag = element.tagName.toLowerCase();
        const role = element.getAttribute("role");
        const visual = style.backgroundColor !== "rgba(0, 0, 0, 0)"
          || style.borderWidth !== "0px"
          || style.boxShadow !== "none"
          || tag === "button"
          || tag === "a"
          || tag === "img"
          || tag === "svg"
          || /^h[1-6]$/.test(tag)
          || role === "button"
          || role === "link";
        const visible = rect.width > 0
          && rect.height > 0
          && style.display !== "none"
          && style.visibility !== "hidden"
          && style.opacity !== "0";
        return visible && visual ? {
          index,
          nodeId: `node_${index + 1}`,
          width: Math.round(rect.width),
          height: Math.round(rect.height)
        } : undefined;
      })
      .filter(Boolean)
      .slice(0, 40) as Array<{ index: number; nodeId: string; width: number; height: number }>;
  });

  const screenshots: ScreenshotFact[] = [];
  for (const [index, target] of targets.entries()) {
    const locator = page.locator("*").nth(target.index);
    const bytes = await locator.screenshot().catch(() => undefined);
    if (!bytes) {
      continue;
    }
    const ordinal = String(index + 1).padStart(3, "0");
    const path = outputDir ? `${outputDir}/element-${ordinal}-${target.nodeId}.png` : undefined;
    if (path) {
      await Bun.write(path, bytes);
    }
    screenshots.push({
      id: `screenshot:element:${viewportSlug(request)}:${ordinal}`,
      type: "element",
      path,
      sha256: await hashBytes(bytes),
      width: target.width,
      height: target.height,
      device_scale_factor: request.viewport.device_scale_factor,
      viewport: request.viewport,
      target_node_id: target.nodeId
    });
  }
  return screenshots;
}

function viewportSlug(request: CaptureRequest): string {
  const dpr = request.viewport.device_scale_factor;
  return `${request.viewport.width}x${request.viewport.height}@${dpr}`;
}

async function hashBytes(bytes: Uint8Array): Promise<string> {
  const digest = await crypto.subtle.digest("SHA-256", bytes);
  return `sha256:${Array.from(new Uint8Array(digest)).map((byte) => byte.toString(16).padStart(2, "0")).join("")}`;
}
