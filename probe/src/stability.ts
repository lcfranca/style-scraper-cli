import type { Page, Response } from "playwright";
import type { CaptureRequest } from "./schema";

export type ProbeDiagnostic = {
  code: string;
  message: string;
  phase: string;
  severity: string;
  recoverable?: boolean;
  recommended_action?: string;
  timings?: Record<string, number>;
  completed?: Record<string, boolean>;
  http_status?: number;
  retry_after?: string;
};

type NavigationOutcome = {
  diagnostics: ProbeDiagnostic[];
  timings: Record<string, number>;
  completed: Record<string, boolean>;
  response?: Response | null;
};

export async function navigateAndWait(
  page: Page,
  request: CaptureRequest
): Promise<NavigationOutcome> {
  page.setDefaultTimeout(request.captureTimeoutMs);
  const diagnostics: ProbeDiagnostic[] = [];
  const timings: Record<string, number> = {};
  const completed: Record<string, boolean> = {
    domcontentloaded: false,
    load: false,
    fonts_ready: false,
    networkidle: false,
    stability_window: false,
    selector: false,
    visible_dom: false
  };

  logProgress("navigation", `goto ${request.url}`);
  let response: Response | null | undefined;
  const gotoStart = performance.now();
  try {
    response = await page.goto(request.url, {
      waitUntil: "domcontentloaded",
      timeout: request.navigationTimeoutMs
    });
    completed.domcontentloaded = true;
    timings.goto_ms = elapsed(gotoStart);
    logProgress("navigation", `domcontentloaded after ${timings.goto_ms}ms`);
  } catch (error) {
    timings.goto_ms = elapsed(gotoStart);
    diagnostics.push(timeoutDiagnostic(
      "NAVIGATION_DOMCONTENTLOADED_TIMEOUT",
      `Initial navigation did not reach DOMContentLoaded within ${request.navigationTimeoutMs}ms.`,
      "navigation",
      timings,
      completed
    ));
    logProgress("navigation", `DOMContentLoaded timeout after ${timings.goto_ms}ms`);
    if (!request.captureOnTimeout || request.strictCapture) {
      throw error;
    }
  }

  diagnostics.push(...httpDiagnostics(response));

  if (request.wait === "selector" || (request.wait === "auto" && request.waitForSelector)) {
    await phase("selector", diagnostics, timings, completed, request, async () => {
      if (!request.waitForSelector) {
        throw new Error("--wait selector requires --wait-for-selector");
      }
      await page.waitForSelector(request.waitForSelector, {
        timeout: Math.min(request.captureTimeoutMs, 5000),
        state: "visible"
      });
    });
  }

  if (["load", "stable", "auto"].includes(request.wait)) {
    await phase("load", diagnostics, timings, completed, request, async () => {
      await page.waitForLoadState("load", { timeout: Math.min(request.captureTimeoutMs, 5000) });
    });
  }

  if (["stable", "auto"].includes(request.wait)) {
    await phase("fonts_ready", diagnostics, timings, completed, request, async () => {
      await page.evaluate((timeoutMs) => Promise.race([
        (document as Document & { fonts?: FontFaceSet }).fonts?.ready ?? Promise.resolve(),
        new Promise((resolve) => window.setTimeout(resolve, timeoutMs))
      ]), Math.min(request.captureTimeoutMs, 3000));
    });
  }

  if (["networkidle", "stable", "auto"].includes(request.wait)) {
    await phase("networkidle", diagnostics, timings, completed, {
      ...request,
      strictCapture: request.strictCapture && !request.ignoreNetworkidleTimeout
    }, async () => {
      await page.waitForLoadState("networkidle", { timeout: Math.min(request.captureTimeoutMs, 5000) });
    });
  }

  if (["stable", "auto"].includes(request.wait)) {
    await phase("stability_window", diagnostics, timings, completed, request, async () => {
      await page.evaluate(({ quietWindowMs, maxWaitMs }) => new Promise<void>((resolve, reject) => {
        const started = performance.now();
        let timer = window.setTimeout(done, quietWindowMs);
        let maxTimer = window.setTimeout(() => {
          observer.disconnect();
          reject(new Error(`stability window not reached within ${maxWaitMs}ms`));
        }, maxWaitMs);
        const observer = new MutationObserver(() => {
          if (performance.now() - started >= maxWaitMs) {
            window.clearTimeout(timer);
            window.clearTimeout(maxTimer);
            observer.disconnect();
            reject(new Error(`stability window not reached within ${maxWaitMs}ms`));
            return;
          }
          window.clearTimeout(timer);
          timer = window.setTimeout(done, quietWindowMs);
        });
        function done() {
          window.clearTimeout(maxTimer);
          observer.disconnect();
          resolve();
        }
        observer.observe(document.documentElement, {
          attributes: true,
          childList: true,
          subtree: true
        });
      }), {
        quietWindowMs: request.stabilityWindowMs,
        maxWaitMs: request.maxStabilityWaitMs
      });
    });
  }

  completed.visible_dom = await hasVisibleDom(page);
  if (!completed.visible_dom) {
    const diagnostic = {
      code: "NO_VISIBLE_DOM_CAPTURED",
      message: "No visible DOM was available after navigation and wait phases.",
      phase: "capture",
      severity: "error",
      recoverable: true,
      timings,
      completed,
      recommended_action: "Try --wait domcontentloaded, provide --auth-state for authorized sessions, or inspect whether the page is blocked."
    };
    diagnostics.push(diagnostic);
    if (!request.captureOnTimeout || request.strictCapture) {
      throw new Error(diagnostic.message);
    }
  }

  if (diagnostics.some((diagnostic) => diagnostic.code.includes("TIMEOUT"))) {
    diagnostics.push({
      code: "CAPTURE_PARTIAL_TIMEOUT",
      message: "One or more non-critical wait phases timed out; visible DOM capture continued.",
      phase: "capture",
      severity: request.strictCapture ? "error" : "warning",
      recoverable: true,
      timings,
      completed,
      recommended_action: "Use --wait domcontentloaded or --capture-on-timeout for pages with persistent network activity."
    });
  }

  return { diagnostics, timings, completed, response };
}

async function phase(
  name: string,
  diagnostics: ProbeDiagnostic[],
  timings: Record<string, number>,
  completed: Record<string, boolean>,
  request: CaptureRequest,
  action: () => Promise<void>
): Promise<void> {
  const started = performance.now();
  logProgress("wait", `${name} started`);
  try {
    await action();
    timings[`${name}_wait_ms`] = elapsed(started);
    completed[name] = true;
    logProgress("wait", `${name} completed after ${timings[`${name}_wait_ms`]}ms`);
  } catch (error) {
    timings[`${name}_wait_ms`] = elapsed(started);
    completed[name] = false;
    diagnostics.push(timeoutDiagnostic(
      `WAIT_${name.toUpperCase()}_TIMEOUT`,
      `${name} did not complete within its phase timeout.`,
      "capture",
      timings,
      completed
    ));
    logProgress("wait", `${name} timed out after ${timings[`${name}_wait_ms`]}ms`);
    if (!request.captureOnTimeout || request.strictCapture) {
      throw error;
    }
  }
}

function timeoutDiagnostic(
  code: string,
  message: string,
  phase: string,
  timings: Record<string, number>,
  completed: Record<string, boolean>
): ProbeDiagnostic {
  return {
    code,
    message,
    phase,
    severity: "warning",
    recoverable: true,
    timings: { ...timings },
    completed: { ...completed },
    recommended_action: "Use --wait auto or --capture-on-timeout for pages with persistent activity."
  };
}

function httpDiagnostics(response?: Response | null): ProbeDiagnostic[] {
  if (!response) {
    return [];
  }
  const status = response.status();
  const retryAfter = response.headers()["retry-after"];
  const diagnostics: ProbeDiagnostic[] = [];
  if ([403, 429, 503].includes(status)) {
    diagnostics.push({
      code: `HTTP_${status}_CAPTURE_WARNING`,
      message: `Initial navigation returned HTTP ${status}. The captured page may be blocked, rate-limited, or unavailable.`,
      phase: "navigation",
      severity: "warning",
      recoverable: true,
      http_status: status,
      retry_after: retryAfter,
      recommended_action: "Use this CLI only on sites you own or have authorization to inspect. Respect Retry-After and provide --auth-state for authorized sessions when appropriate."
    });
  }
  return diagnostics;
}

async function hasVisibleDom(page: Page): Promise<boolean> {
  return page.evaluate(() => {
    const elements = Array.from(document.querySelectorAll("body *"));
    return elements.some((element) => {
      const rect = element.getBoundingClientRect();
      const style = getComputedStyle(element);
      return rect.width > 0
        && rect.height > 0
        && style.display !== "none"
        && style.visibility !== "hidden"
        && style.opacity !== "0";
    });
  }).catch(() => false);
}

function elapsed(started: number): number {
  return Math.round(performance.now() - started);
}

export function logProgress(phase: string, message: string): void {
  process.stderr.write(`style-scraper probe | ${phase} | ${message}\n`);
}
