import { chromium, type Page } from "playwright";
import { parseCliArgs } from "./cli";
import { captureAccessibilityTree } from "./aom";
import { collectObservedPage } from "./dom";
import { captureScreenshots } from "./screenshots";
import { logProgress, navigateAndWait } from "./stability";
import { RawFactsSchema } from "./schema";
import { captureStateDeltas } from "./states";
import { installResourcePolicy } from "./resources";

async function main(): Promise<void> {
  const request = parseCliArgs();
  logProgress("startup", "style-scraper probe booting");
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
  const resourceDiagnostics = await installResourcePolicy(context, request);

  try {
    const page = await context.newPage();
    const navigation = await navigateAndWait(page, request);

    const nonFatalDiagnostics: Array<Record<string, unknown>> = [];
    logProgress("capture", "capturing safe visual states");
    const stateDeltas = await withTimeout(
      captureStateDeltas(page, request.states, {
        safeInteractions: request.safeInteractions,
        noClick: request.noClick,
        noFormSubmit: request.noFormSubmit,
        allowActive: request.allowActive,
        clickSelector: request.clickSelector
      }),
      Math.min(request.captureTimeoutMs, 5000),
      "state capture"
    ).catch((error) => {
      nonFatalDiagnostics.push({
        code: "STATE_CAPTURE_TIMEOUT",
        message: error instanceof Error ? error.message : String(error),
        phase: "capture",
        severity: "warning",
        recoverable: true,
        recommended_action: "Use --states '' or fewer states for pages with expensive interactive handlers."
      });
      return [];
    });
    logProgress("capture", "capturing DOM/CSSOM/layout/AOM evidence");
    const observed = await collectObservedPage(page, request);
    const accessibilityTree = await captureAccessibilityTree(page);
    logProgress("capture", "capturing screenshot artifacts");
    const screenshots = await captureScreenshots(page, request);
    const challengeDiagnostics = await detectBlockOrChallengePage(page);

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
        viewport: request.viewport,
        dom: observed.dom,
        cssom: observed.cssom,
        layout: observed.layout,
        accessibility: {
          nodes: observed.accessibility.nodes,
          tree: accessibilityTree ?? undefined
        },
        screenshots,
        pseudo_elements: observed.pseudo_elements,
        assets: observed.assets,
        stylesheet_provenance: observed.stylesheet_provenance,
        state_deltas: stateDeltas
      }],
      diagnostics: [
        ...resourceDiagnostics,
        ...navigation.diagnostics,
        ...observed.diagnostics,
        ...challengeDiagnostics,
        ...nonFatalDiagnostics,
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
    logProgress("capture", "writing RawFacts JSON");
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

function withTimeout<T>(promise: Promise<T>, timeoutMs: number, label: string): Promise<T> {
  return Promise.race([
    promise,
    new Promise<T>((_, reject) => {
      setTimeout(() => reject(new Error(`${label} exceeded ${timeoutMs}ms`)), timeoutMs);
    })
  ]);
}

async function detectBlockOrChallengePage(page: Page) {
  const result = await page.evaluate(() => {
    const title = document.title.toLowerCase();
    const body = (document.body?.innerText ?? "").toLowerCase().slice(0, 5000);
    const hasCaptcha = Boolean(document.querySelector("[class*='captcha' i], [id*='captcha' i], iframe[src*='captcha' i]"));
    const challengeText = [
      "captcha",
      "cloudflare",
      "checking your browser",
      "access denied",
      "temporarily unavailable",
      "unusual traffic",
      "verify you are human",
      "challenge"
    ].some((needle) => title.includes(needle) || body.includes(needle));
    return { detected: hasCaptcha || challengeText, title: document.title };
  }).catch(() => ({ detected: false, title: "" }));

  if (!result.detected) {
    return [];
  }
  return [{
    code: "BOT_PROTECTION_OR_BLOCK_PAGE_DETECTED",
    message: "The captured page appears to be a bot-protection, WAF, CAPTCHA, or challenge page.",
    phase: "capture",
    severity: "warning",
    recoverable: true,
    recommended_action: "Use this CLI only on sites you own or have authorization to inspect. Provide --auth-state for authorized sessions when appropriate."
  }];
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
