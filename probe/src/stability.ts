import type { Page } from "playwright";
import type { CaptureRequest } from "./schema";

export async function waitForRenderStability(
  page: Page,
  request: CaptureRequest
): Promise<void> {
  page.setDefaultTimeout(request.timeoutMs);

  if (request.wait === "load") {
    await page.waitForLoadState("load", { timeout: request.timeoutMs });
  } else if (request.wait === "networkidle") {
    await page.waitForLoadState("networkidle", { timeout: request.timeoutMs });
  } else if (request.wait === "stable") {
    await page.waitForLoadState("networkidle", { timeout: Math.min(request.timeoutMs, 8000) }).catch(() => undefined);
    await page.evaluate(async () => {
      await (document as Document & { fonts?: FontFaceSet }).fonts?.ready;
    }).catch(() => undefined);
    await page.evaluate(() => new Promise<void>((resolve) => {
      const quietWindowMs = 500;
      let timer = window.setTimeout(done, quietWindowMs);
      const observer = new MutationObserver(() => {
        window.clearTimeout(timer);
        timer = window.setTimeout(done, quietWindowMs);
      });
      function done() {
        observer.disconnect();
        resolve();
      }
      observer.observe(document.documentElement, {
        attributes: true,
        childList: true,
        subtree: true
      });
    })).catch(() => undefined);
  }
}
