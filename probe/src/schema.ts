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
  wait: z.enum(["domcontentloaded", "load", "networkidle", "stable"]).default("stable"),
  timeoutMs: z.number().int().positive().default(30000),
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
    dom: z.object({ nodes: z.array(z.unknown()) }),
    cssom: z.object({ computed_styles: z.array(z.unknown()) }),
    layout: z.object({ boxes: z.array(z.unknown()) }),
    accessibility: z.object({
      nodes: z.array(z.unknown()),
      tree: z.unknown().optional()
    }),
    screenshots: z.array(z.unknown()).default([]),
    state_deltas: z.array(z.unknown()).default([])
  })).min(1),
  diagnostics: z.array(z.object({
    code: z.string(),
    message: z.string(),
    phase: z.string(),
    severity: z.string()
  })).default([])
});

export type RawFacts = z.infer<typeof RawFactsSchema>;
