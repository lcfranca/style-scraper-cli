# style-scraper Corporate Roadmap

## Phase 1: Scaffold And Contracts

Acceptance: Rust remains the analytical CLI, Bun/TypeScript/Playwright remains the probe, `style-scraper` is the binary, no internal LLM/generative AI/remote embedding dependency exists, and schemas/ADRs define raw facts, final output, tokens, morphology, diagnostics, errors, no-ops execution, and anti-abuse policy.

Risk: weak contracts force every later phase to reinterpret the runtime boundary.

## Phase 2: Production Browser Probe

Acceptance: the probe captures visible DOM, relevant attributes, computed CSSOM, bounding boxes, viewport/device scale/color scheme, AOM, optional screenshots, open Shadow DOM, accessible iframes, safe hover/focus/focus-visible states, render stability, structured errors, and text redaction/hashing.

Risk: local environments without Bun, Playwright packages, or browser binaries prevent end-to-end validation.

## Phase 3: Corporate Rust Probe Runner

Acceptance: Rust detects Bun, Playwright, and browser availability failures; enforces subprocess timeout; separates stdout/stderr; uses file IPC for large JSON; cleans temporary files unless requested; propagates exit codes; and logs only to stderr.

Risk: large subprocess output can deadlock if file or pipe handling regresses.

## Phase 4: Rust Normalization And Validation

Acceptance: serde models, schema validation, deterministic color/unit/type/spacing/box normalization, stable IDs, stable ordering, deterministic hashes, and reproducibility manifest are covered by tests.

Risk: timestamped or environment-specific fields can destabilize golden outputs.

## Phase 5: W3C Token Engine

Acceptance: core, semantic, component, theme, responsive, color, typography, spacing, radius, border, shadow/elevation, opacity, z-index, and motion tokens include `$type`, `$value`, optional `$description`, evidence, frequency, confidence, and epistemic source.

Risk: over-semantic naming can overstate evidence.

## Phase 6: Gestalt And Atomic Morphology

Acceptance: spatial indexing, proximity, alignment, visual similarity, common region, DOM/AOM reconciliation, atom/molecule/organism/template/page classification, confidence scoring, and evidence graph are implemented.

Risk: broad containers can produce noisy clusters without careful scoring.

## Phase 7: Safe Crawl

Acceptance: crawl is opt-in and bounded by `--max-pages`, `--max-depth`, `--same-origin`, `--respect-robots`, `--delay-ms`, `--parallel-pages`, include/exclude paths, URL deduplication/canonicalization, dangerous route exclusion, and 429/403/503/Retry-After stop/backoff.

Risk: crawl can become abusive if limits are bypassed.

## Phase 8: Privacy And Anti-Abuse

Acceptance: `--redact-text`, `--hash-text`, `--include-text`, `--redact-attributes`, `--redact-images`, `--safe-interactions`, `--no-click`, `--no-form-submit`, `--auth-state`, artifact manifest, and authorized-use docs are present.

Risk: authenticated pages can expose sensitive information if defaults regress.

## Phase 9: Advanced Modes

Acceptance: capture, analyze, tokens, diff, validate, explain, crawl, storybook, component-library, responsive extraction, light/dark themes, artifact directory, compact payload, and evidence-level modes are implemented incrementally.

Risk: command breadth can outpace core correctness.

## Phase 10: Quality

Acceptance: Rust and TypeScript unit tests, local fixture integration tests, golden snapshots, determinism tests, schema tests, performance benchmarks, large DOM tests, visual tests, crawl safety tests, error contract tests, CI, fmt, clippy, test, audit, Bun checks, and Playwright install checks exist.

Risk: browser-based CI is heavier than pure Rust CI.

## Phase 11: Distribution

Acceptance: release Rust binary, Bun probe installation guidance, bundled probe strategy, Docker image with Bun and Playwright browsers, checksums, SBOM, semantic versioning, changelog, corporate README, examples, troubleshooting, and security policy exist.

Risk: distribution claims must match actual browser packaging.
