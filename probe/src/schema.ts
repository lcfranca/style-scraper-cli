import { z } from "zod";

export const ViewportSchema = z.object({
  width: z.number().int().positive(),
  height: z.number().int().positive(),
  device_scale_factor: z.number().positive().default(1)
});

export const CaptureRequestSchema = z.object({
  url: z.string().url(),
  viewport: ViewportSchema,
  colorScheme: z.enum(["light", "dark"]).default("light"),
  states: z.array(z.string()).default([]),
  includeScreenshots: z.boolean().default(false),
  screenshot: z.enum(["viewport", "full-page", "elements"]).default("viewport"),
  wait: z.enum(["domcontentloaded", "load", "networkidle", "stable", "selector", "auto"]).default("auto"),
  waitForSelector: z.string().optional(),
  timeoutMs: z.number().int().positive().default(30000),
  navigationTimeoutMs: z.number().int().positive().default(15000),
  captureTimeoutMs: z.number().int().positive().default(30000),
  stabilityWindowMs: z.number().int().positive().default(500),
  maxStabilityWaitMs: z.number().int().positive().default(5000),
  ignoreNetworkidleTimeout: z.boolean().default(true),
  captureOnTimeout: z.boolean().default(true),
  strictCapture: z.boolean().default(false),
  safeCapture: z.boolean().default(false),
  resourceBudget: z.enum(["safe", "balanced", "full"]).default("balanced"),
  blockThirdParty: z.boolean().default(false),
  blockAnalytics: z.boolean().default(false),
  blockMedia: z.boolean().default(false),
  blockFonts: z.boolean().default(false),
  blockImages: z.boolean().default(false),
  blockDownloads: z.boolean().default(true),
  allowActive: z.boolean().default(false),
  clickSelector: z.string().optional(),
  authState: z.string().optional(),
  screenshotDir: z.string().optional(),
  outputFile: z.string().optional(),
  redactText: z.boolean().default(false),
  hashText: z.boolean().default(true),
  includeText: z.boolean().default(false),
  redactAttributes: z.boolean().default(false),
  redactImages: z.boolean().default(false),
  safeInteractions: z.boolean().default(true),
  noClick: z.boolean().default(true),
  noFormSubmit: z.boolean().default(true)
});

export type CaptureRequest = z.infer<typeof CaptureRequestSchema>;

export const RawFactsSchema = z.object({
  schema_version: z.literal("raw-facts.v1"),
  url: z.string().url(),
  viewport: ViewportSchema,
  color_scheme: z.enum(["light", "dark"]).optional(),
  captured_at: z.string().optional(),
  runtime: z.literal("bun"),
  browser: z.object({
    name: z.string(),
    version: z.string().optional(),
    user_agent_hash: z.string().optional()
  }),
  pages: z.array(z.object({
    url: z.string(),
    title: z.string(),
    viewport: ViewportSchema.optional(),
    dom: z.object({ nodes: z.array(z.unknown()) }),
    cssom: z.object({ computed_styles: z.array(z.unknown()) }),
    layout: z.object({ boxes: z.array(z.unknown()) }),
    accessibility: z.object({
      nodes: z.array(z.unknown()),
      tree: z.unknown().optional()
    }),
    screenshots: z.array(z.unknown()).default([]),
    pseudo_elements: z.array(z.unknown()).default([]),
    assets: z.array(z.unknown()).default([]),
    stylesheet_provenance: z.unknown().optional(),
    state_deltas: z.array(z.unknown()).default([])
  })).min(1),
  diagnostics: z.array(z.object({
    code: z.string(),
    message: z.string(),
    phase: z.string(),
    severity: z.string()
  }).passthrough()).default([])
});

export type RawFacts = z.infer<typeof RawFactsSchema>;
