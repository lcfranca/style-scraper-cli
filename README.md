# style-scraper

`style-scraper` is a local, ephemeral, no-ops CLI that captures browser-rendered frontend facts and turns them into deterministic, evidence-backed design token and morphology JSON for external AI agents.

The architecture is deliberately split:

- Rust is the brain: CLI, orchestration, validation, normalization, token inference, morphology, Gestalt grouping, AOM reconciliation, and final JSON emission.
- Bun + TypeScript + Playwright are the eyes: browser launch, DOM/CSSOM/layout/AOM/screenshot capture, and `RawFacts` JSON output.

The tool does not contain or call LLMs, generative AI APIs, remote embeddings, autonomous agents, or business-goal decision logic. It is a deterministic sensory and analytical provider for external callers.

## Development

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

`stdout` is reserved for strict machine-readable output. Diagnostics and failures go to `stderr`.

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
- `NAVIGATION_TIMEOUT`: increase `--timeout-ms`, use a narrower target page, or inspect target availability.
