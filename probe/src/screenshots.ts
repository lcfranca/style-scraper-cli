import { mkdir } from "node:fs/promises";
import type { Page } from "playwright";
import type { CaptureRequest } from "./schema";

export async function captureScreenshots(
  page: Page,
  request: CaptureRequest
): Promise<unknown[]> {
  if (!request.includeScreenshots || request.redactImages) {
    return [];
  }

  const bytes = await page.screenshot({ fullPage: false });
  const sha256 = await hashBytes(bytes);
  let path: string | undefined;
  if (request.screenshotDir) {
    await mkdir(request.screenshotDir, { recursive: true });
    path = `${request.screenshotDir.replace(/\/$/, "")}/viewport.png`;
    await Bun.write(path, bytes);
  }

  return [{
    id: "screenshot:viewport",
    screenshot_type: "viewport",
    path,
    sha256,
    viewport: request.viewport
  }];
}

async function hashBytes(bytes: Uint8Array): Promise<string> {
  const digest = await crypto.subtle.digest("SHA-256", bytes);
  return `sha256:${Array.from(new Uint8Array(digest)).map((byte) => byte.toString(16).padStart(2, "0")).join("")}`;
}
