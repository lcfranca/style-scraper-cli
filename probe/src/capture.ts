import { chromium } from "playwright";
import { parseCliArgs } from "./cli";
import { captureAccessibilityTree } from "./aom";
import { collectObservedPage } from "./dom";
import { captureScreenshots } from "./screenshots";
import { waitForRenderStability } from "./stability";
import { RawFactsSchema } from "./schema";
import { captureStateDeltas } from "./states";

async function main(): Promise<void> {
  const request = parseCliArgs();
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({
    viewport: {
      width: request.viewport.width,
      height: request.viewport.height
    },
    deviceScaleFactor: request.viewport.device_scale_factor,
    colorScheme: request.colorScheme,
    storageState: request.authState
  });

  try {
    const page = await context.newPage();
    await page.goto(request.url, {
      waitUntil: request.wait === "load" ? "load" : "domcontentloaded",
      timeout: request.timeoutMs
    });
    await waitForRenderStability(page, request);

    const stateDeltas = await captureStateDeltas(page, request.states, {
      safeInteractions: request.safeInteractions,
      noClick: request.noClick,
      noFormSubmit: request.noFormSubmit
    });
    const observed = await collectObservedPage(page, request);
    const accessibilityTree = await captureAccessibilityTree(page);
    const screenshots = await captureScreenshots(page, request);

    const rawFacts = {
      schema_version: "raw-facts.v1",
      url: observed.url,
      viewport: request.viewport,
      color_scheme: request.colorScheme,
      captured_at: new Date().toISOString(),
      runtime: "bun",
      browser: {
        name: "chromium",
        version: browser.version(),
        user_agent_hash: observed.user_agent_hash
      },
      pages: [{
        url: observed.url,
        title: observed.title,
        dom: observed.dom,
        cssom: observed.cssom,
        layout: observed.layout,
        accessibility: {
          nodes: observed.accessibility.nodes,
          tree: accessibilityTree ?? undefined
        },
        screenshots,
        state_deltas: stateDeltas
      }],
      diagnostics: [
        ...observed.diagnostics,
        ...(accessibilityTree ? [] : [{
          code: "AOM_TREE_UNAVAILABLE",
          message: "Playwright accessibility snapshot was unavailable; probe emitted DOM-mapped accessibility role evidence only.",
          phase: "capture",
          severity: "warning"
        }])
      ]
    };

    const validated = RawFactsSchema.parse(rawFacts);
    const json = JSON.stringify(validated);
    if (request.outputFile) {
      await Bun.write(request.outputFile, json);
    } else {
      process.stdout.write(json);
    }
  } finally {
    await context.close().catch(() => undefined);
    await browser.close().catch(() => undefined);
  }
}

main().catch(async (error: unknown) => {
  const message = error instanceof Error ? error.message : String(error);
  const isTimeout = /timeout/i.test(message);
  const isPlaywrightMissing = /Cannot find package 'playwright'|Cannot find module 'playwright'|Module not found/i.test(message);
  const isBrowserMissing = /Executable doesn't exist|playwright install|browserType.launch/i.test(message);
  process.stderr.write(JSON.stringify({
    error: {
      code: isTimeout
        ? "NAVIGATION_TIMEOUT"
        : isPlaywrightMissing
          ? "PLAYWRIGHT_NOT_FOUND"
          : isBrowserMissing
            ? "BROWSER_NOT_INSTALLED"
            : "PROBE_FAILURE",
      message,
      phase: "capture",
      recoverable: true
    }
  }));
  process.exit(isTimeout ? 4 : 3);
});
