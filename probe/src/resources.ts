import type { BrowserContext, Route } from "playwright";
import type { CaptureRequest } from "./schema";
import type { ProbeDiagnostic } from "./stability";

const ANALYTICS_HOST_PATTERNS = [
  "google-analytics.com",
  "googletagmanager.com",
  "doubleclick.net",
  "facebook.net",
  "facebook.com/tr",
  "hotjar.com",
  "clarity.ms",
  "segment.io",
  "mixpanel.com",
  "amplitude.com",
  "newrelic.com",
  "datadoghq-browser-agent.com"
];

export async function installResourcePolicy(
  context: BrowserContext,
  request: CaptureRequest
): Promise<ProbeDiagnostic[]> {
  const diagnostics: ProbeDiagnostic[] = [];
  const blocksAnalytics = request.resourceBudget !== "full" || request.blockAnalytics;
  const blocksMedia = request.resourceBudget === "safe" || request.blockMedia;
  const blocksThirdParty = request.blockThirdParty;

  diagnostics.push({
    code: "RESOURCE_BUDGET_ACTIVE",
    message: `Resource budget '${request.resourceBudget}' is active. Analytics=${blocksAnalytics}; media=${blocksMedia}; third_party=${blocksThirdParty}.`,
    phase: "capture",
    severity: request.resourceBudget === "safe" ? "warning" : "info",
    recoverable: true,
    recommended_action: request.resourceBudget === "safe"
      ? "Use --resource-budget full when visual fidelity is more important than conservative loading."
      : undefined
  });

  await context.route("**/*", async (route: Route) => {
    const resource = route.request();
    const url = resource.url();
    const type = resource.resourceType();
    const isAnalytics = ANALYTICS_HOST_PATTERNS.some((pattern) => url.includes(pattern));
    const isMedia = type === "media";
    const isFont = type === "font";
    const isImage = type === "image";
    const isDownloadLike = /\.(zip|dmg|pkg|exe|msi|iso|tar|gz|rar|7z)(?:[?#]|$)/i.test(url);
    const isThirdParty = thirdParty(url, request.url);

    if (request.blockDownloads && isDownloadLike) {
      await route.abort("blockedbyclient");
      return;
    }
    if (blocksAnalytics && isAnalytics) {
      await route.abort("blockedbyclient");
      return;
    }
    if ((blocksMedia && isMedia) || (request.blockFonts && isFont) || (request.blockImages && isImage)) {
      await route.abort("blockedbyclient");
      return;
    }
    if (blocksThirdParty && isThirdParty && type !== "document") {
      await route.abort("blockedbyclient");
      return;
    }
    await route.continue();
  });

  return diagnostics;
}

function thirdParty(url: string, targetUrl: string): boolean {
  try {
    return new URL(url).origin !== new URL(targetUrl).origin;
  } catch {
    return false;
  }
}
