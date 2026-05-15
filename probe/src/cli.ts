import { CaptureRequest, CaptureRequestSchema } from "./schema";

export function parseCliArgs(argv = Bun.argv.slice(2)): CaptureRequest {
  const args = new Map<string, string | boolean>();
  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (!current.startsWith("--")) {
      continue;
    }
    const key = current.slice(2);
    const next = argv[index + 1];
    if (!next || next.startsWith("--")) {
      args.set(key, true);
    } else {
      args.set(key, next);
      index += 1;
    }
  }

  const url = stringArg(args, "url");
  const viewport = parseViewport(stringArg(args, "viewport") ?? "1440x900");
  const states = (stringArg(args, "states") ?? "")
    .split(",")
    .map((state) => state.trim())
    .filter(Boolean);

  return CaptureRequestSchema.parse({
    url,
    viewport,
    colorScheme: stringArg(args, "color-scheme") ?? "light",
    states,
    includeScreenshots: args.get("include-screenshots") === true,
    screenshot: stringArg(args, "screenshot") ?? "viewport",
    wait: stringArg(args, "wait") ?? "auto",
    waitForSelector: stringArg(args, "wait-for-selector"),
    timeoutMs: Number(stringArg(args, "timeout-ms") ?? "30000"),
    navigationTimeoutMs: Number(stringArg(args, "navigation-timeout-ms") ?? "15000"),
    captureTimeoutMs: Number(stringArg(args, "capture-timeout-ms") ?? "30000"),
    stabilityWindowMs: Number(stringArg(args, "stability-window-ms") ?? "500"),
    maxStabilityWaitMs: Number(stringArg(args, "max-stability-wait-ms") ?? "5000"),
    ignoreNetworkidleTimeout: args.get("ignore-networkidle-timeout") !== false,
    captureOnTimeout: args.get("capture-on-timeout") !== false,
    strictCapture: args.get("strict-capture") === true,
    safeCapture: args.get("safe-capture") === true,
    resourceBudget: stringArg(args, "resource-budget") ?? "balanced",
    blockThirdParty: args.get("block-third-party") === true,
    blockAnalytics: args.get("block-analytics") === true,
    blockMedia: args.get("block-media") === true,
    blockFonts: args.get("block-fonts") === true,
    blockImages: args.get("block-images") === true,
    blockDownloads: true,
    allowActive: args.get("allow-active") === true,
    clickSelector: stringArg(args, "click-selector"),
    authState: stringArg(args, "auth-state"),
    screenshotDir: stringArg(args, "screenshot-dir"),
    outputFile: stringArg(args, "output-file"),
    redactText: args.get("redact-text") === true,
    hashText: args.get("hash-text") === true || args.get("include-text") !== true,
    includeText: args.get("include-text") === true,
    redactAttributes: args.get("redact-attributes") === true,
    redactImages: args.get("redact-images") === true,
    safeInteractions: true,
    noClick: true,
    noFormSubmit: true
  });
}

function stringArg(args: Map<string, string | boolean>, key: string): string | undefined {
  const value = args.get(key);
  return typeof value === "string" ? value : undefined;
}

function parseViewport(value: string): CaptureRequest["viewport"] {
  const match = /^(\d+)x(\d+)(?:@([0-9.]+))?$/.exec(value);
  if (!match) {
    throw new Error(`Invalid viewport: ${value}`);
  }
  return {
    width: Number(match[1]),
    height: Number(match[2]),
    device_scale_factor: match[3] ? Number(match[3]) : 1
  };
}
