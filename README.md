# style-scraper

`style-scraper` is a local, ephemeral, no-ops CLI that captures browser-rendered frontend facts and turns them into deterministic, evidence-backed design token, morphology, layout, visual, and reconstruction JSON for external AI agents.

The architecture is deliberately split:

- Rust is the brain: CLI, orchestration, validation, normalization, token inference, morphology, Gestalt grouping, AOM reconciliation, and final JSON emission.
- Bun + TypeScript + Playwright are the eyes: browser launch, DOM/CSSOM/layout/AOM/screenshot capture, and `RawFacts` JSON output.

The tool does not contain or call LLMs, generative AI APIs, remote embeddings, autonomous agents, or business-goal decision logic. It is a deterministic sensory and analytical provider for external callers.

## Development

You can automate development tasks using the provided `Makefile`.

```bash
# Display all available commands
make help

# Build the project in debug mode
make build

# Build the project in release mode
make compile

# Run the test suite
make test

# Compile and install the final binary to ~/.cargo/bin (updating current version)
make install
```

Or you can use cargo natively:
```bash
cargo build
cargo run -p style-scraper-cli -- extract --url https://example.com
cd probe && bun install
bun run src/capture.ts --url https://example.com
```

Primary command:

```bash
style-scraper extract --url https://example.com --scope page --detail all --format json
```

### Complete Comprehensive Extraction
For the most robust and complete analytical output against a target, you can use the following functional and self-contained command. It triggers the highest detail level, screenshots, layout inference, interaction states (generating the evidence required for full styling and UI reconstruction payload):

```bash
style-scraper extract \
  --url "https://www.amazon.com" \
  --scope page \
  --detail all \
  --format json \
  --include-screenshots \
  --viewport 1440x900 \
  --color-scheme light \
  --states hover,focus-visible \
  --screenshot-dir .style-scraper/artifacts/screenshots \
  --pretty
```

`stdout` is reserved for strict machine-readable output. Diagnostics and failures go to `stderr`.

## Reconstruction Model

The default JSON is an evidence bundle, not a claim of perfect 1:1 reconstruction. Reconstructive confidence improves when the capture includes pixel evidence, component-token bindings, pseudo-elements, assets, state deltas, and layout constraints.

```bash
style-scraper extract \
  --url https://example.com \
  --include-screenshots \
  --screenshot viewport \
  --states hover,focus-visible \
  --format reconstruction-json
```

Use `--screenshot full-page` for page-height evidence, or `--screenshot elements` to additionally capture relevant element crops. Screenshots are written to `.style-scraper/artifacts/screenshots` by default and referenced in stdout by path and SHA-256 hash; base64 is not embedded.

Responsive extraction is opt-in:

```bash
style-scraper extract \
  --url https://example.com \
  --viewports 375x812,768x1024,1440x900 \
  --include-screenshots
```

The output separates viewport evidence and reports responsive candidates without mixing mobile and desktop facts as if they were one invariant design system.

## Visual Diff

Pixel comparison is the evidence gate for any 1:1 reconstruction claim:

```bash
style-scraper diff-visual --before original.png --after reconstructed.png --pretty
```

The command emits `style-scraper-visual-diff.v1` with `pixel_diff_ratio`, global luminance `ssim`, changed bounds, tolerance, score, thresholds, and verdict.

## Timeout And Real-World Sites

Real pages often never reach `networkidle`: analytics, tracking pixels, long polling, chat widgets, lazy loading, slow third-party fonts, or persistent scripts can keep network and DOM activity alive indefinitely. `style-scraper` defaults to `--wait auto`, which captures useful browser facts after progressive wait phases instead of treating every non-critical timeout as fatal.

Wait modes:

- `domcontentloaded`: fastest; captures after the initial HTML is parsed.
- `load`: waits for browser `load`.
- `networkidle`: strict and often unsuitable for production marketing sites.
- `stable`: waits for load, fonts, network idle, and a mutation quiet window.
- `selector`: waits for `--wait-for-selector`.
- `auto`: robust default; tries load, fonts, network idle, and stability, then captures visible DOM with warnings when non-critical phases time out.

For sensitive or flaky sites, use safe capture:

```bash
style-scraper extract \
  --url "https://www.implanta.net.br" \
  --safe-capture \
  --wait auto \
  --capture-on-timeout \
  --timeout-ms 90000 \
  --navigation-timeout-ms 20000 \
  --max-stability-wait-ms 5000 \
  --include-screenshots \
  --pretty
```

Minimal fallback for difficult pages:

```bash
style-scraper extract \
  --url "https://www.implanta.net.br" \
  --wait domcontentloaded \
  --capture-on-timeout \
  --safe-capture \
  --pretty
```

`--safe-capture` is compliance-first, not stealth-first. It keeps page scope, disables clicks/form submission, blocks downloads, uses conservative resource handling, and may block analytics depending on the resource budget. It does not implement proxy rotation, fingerprint spoofing, CAPTCHA bypass, Cloudflare bypass, WAF bypass, or deceptive stealth scraping.

Resource budgets:

- `--resource-budget full`: best fidelity, fewer blocked resources, slower on noisy pages.
- `--resource-budget balanced`: default; blocks known analytics/tracker endpoints and downloads while preserving visual resources.
- `--resource-budget safe`: more conservative; may block heavier resources and can reduce visual fidelity.

HTTP `403`, `429`, `503`, `Retry-After`, CAPTCHA, WAF, or challenge-page signals are reported as diagnostics. The tool stops at diagnosis; it does not try to solve or bypass protections. Use `--auth-state` only for sessions you are authorized to inspect.

## Epistemic Limits

Computed CSSOM is the primary style source because it reflects what the browser actually rendered. Stylesheet provenance, CSS variables, pseudo-elements, assets, AOM roles, screenshots, and state deltas are secondary evidence. The CLI does not infer business intent, does not name components with an LLM, and does not generate UI by itself.

## Local Workspace Install

During development, install the Rust CLI into a workspace-local binary directory and expose it through `PATH` without requiring a global system install:

```bash
mkdir -p .bin && \
  cargo build --release -p style-scraper-cli && \
  ln -sf "$(pwd)/target/release/style-scraper-cli" .bin/style-scraper && \
  export PATH="$(pwd)/.bin:$PATH" && \
  style-scraper --help
```

For a persistent shell setup inside this workspace, add the workspace-local `.bin` directory to your shell profile or use a project environment manager such as `direnv`.

Example with `direnv`:

```bash
printf 'export PATH="$PWD/.bin:$PATH"\n' > .envrc && \
  direnv allow
```

After this, developers can run:

```bash
style-scraper extract --url https://example.com --scope page --detail all --format json
```

## Developer Install Script

Use this command to create a simple installer script for other developers working in the repository:

```bash
mkdir -p scripts && cat > scripts/install-local.sh <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN_DIR="$ROOT_DIR/.bin"
TARGET_BIN="$ROOT_DIR/target/release/style-scraper-cli"
LINK_BIN="$BIN_DIR/style-scraper"

cd "$ROOT_DIR"
mkdir -p "$BIN_DIR"

cargo build --release -p style-scraper-cli
ln -sf "$TARGET_BIN" "$LINK_BIN"

cat <<MSG
style-scraper installed for this workspace.

Binary symlink:
  $LINK_BIN

To use it in the current shell, run:
  export PATH="$BIN_DIR:\$PATH"

Or enable it automatically with direnv:
  printf 'export PATH="\$PWD/.bin:\$PATH"\\n' > .envrc
  direnv allow

Test:
  style-scraper --help
MSG
EOF
chmod +x scripts/install-local.sh
```

Then run:

```bash
./scripts/install-local.sh
export PATH="$(pwd)/.bin:$PATH"
style-scraper --help
```

## Dependency Bootstrap

The Rust binary is the primary executable. Browser capture also requires Bun, the probe dependencies, and a Playwright browser. By default, `style-scraper extract` and `style-scraper capture` auto-install missing probe dependencies into the workspace, using pinned versions from [probe/requirements.json](/Users/lucasfranca/Workspace/style-scraper-cli/probe/requirements.json):

```bash
style-scraper extract --url https://example.com
```

Installed artifacts are kept under `.style-scraper/deps` by default:

- local Bun runtime when `bun` is not already on `PATH`;
- `probe/node_modules` through `bun install`;
- Playwright browsers under `.style-scraper/deps/playwright-browsers`.

To disable automatic dependency installation:

```bash
style-scraper extract --url https://example.com --no-install-deps
```

Manual installation remains available:

```bash
cd probe
../.style-scraper/deps/bun/bun-v1.1.6/bun-*/bun install
PLAYWRIGHT_BROWSERS_PATH=../.style-scraper/deps/playwright-browsers ../.style-scraper/deps/bun/bun-v1.1.6/bun-*/bun x playwright install chromium
```

## Privacy Defaults

Text is hashed by default. Use `--redact-text` to omit hashes and raw values, or `--include-text` only when authorized content capture is required. `--redact-attributes` and `--redact-images` reduce sensitive attribute and screenshot exposure.

Safe interactions are the default. The probe may use hover/focus/focus-visible for state deltas, but clicks and form submissions are disabled by default.

## Safe Crawl Contract

Page extraction is the default. Crawl is opt-in and bounded. The current CLI exposes the safe crawl contract with conservative defaults while route discovery and aggregation are still pending:

```bash
style-scraper crawl --url https://example.com --max-pages 25 --max-depth 2 --same-origin
```

The project explicitly rejects stealth scraping, proxy rotation, CAPTCHA bypass, credential collection, and hidden remote extraction.

## Troubleshooting

- `BUN_NOT_FOUND`: install Bun or use a future Docker image.
- `PLAYWRIGHT_NOT_FOUND`: run `cd probe && bun install`.
- `BROWSER_NOT_INSTALLED`: run `cd probe && bunx playwright install chromium`.
- `WAIT_NETWORKIDLE_TIMEOUT`: usually non-fatal under `--wait auto`; the page kept network activity alive and capture continued if visible DOM existed.
- `CAPTURE_PARTIAL_TIMEOUT`: non-critical phase timeout; inspect warnings and screenshots before deciding whether the capture is sufficient.
- `PROBE_WATCHDOG_TIMEOUT`: the whole Bun subprocess stopped making progress before RawFacts were written; reduce states/screenshots, increase `--capture-timeout-ms`, or use `--wait domcontentloaded`.
- `HTTP_403_CAPTURE_WARNING`, `HTTP_429_CAPTURE_WARNING`, `HTTP_503_CAPTURE_WARNING`: target may be blocked, rate-limited, or unavailable. Respect `Retry-After` and authorization boundaries.
