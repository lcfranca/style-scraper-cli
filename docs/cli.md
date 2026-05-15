# style-scraper CLI

## Command Surface

```bash
style-scraper extract --url https://example.com --scope page --detail all --format json
style-scraper capture --url https://example.com --raw-output raw-facts.json
style-scraper analyze --input raw-facts.json --format json
style-scraper tokens --input raw-facts.json --format w3c-tokens
style-scraper diff --before old.json --after new.json
style-scraper validate --input output.json
style-scraper crawl --url https://example.com --max-pages 25 --same-origin
```

## Defaults

- `--scope page`
- `--detail all`
- `--format json`
- `--viewport 1440x900`
- `--color-scheme light`
- `--browser chromium`
- `--wait stable`
- text privacy: hash text by default
- interactions: hover/focus only by default; no clicks or form submissions

`stdout` contains only JSON for all implemented machine-facing formats. Diagnostics and structured errors are written to `stderr`.

## Privacy And Artifacts

```bash
style-scraper extract --url https://example.com --redact-text --redact-attributes
style-scraper extract --url https://example.com --include-screenshots --screenshot-dir artifacts/screenshots
style-scraper extract --url https://example.com --include-text
style-scraper extract --url https://example.com --keep-artifacts --artifact-dir .style-scraper/artifacts
```

Raw text capture is opt-in via `--include-text`.

## Dependency Installation

The CLI auto-installs missing probe dependencies by default, similar to applying a pinned requirements file:

```bash
style-scraper extract --url https://example.com
style-scraper capture --url https://example.com --raw-output raw.json
```

Use these flags to control the bootstrap:

```bash
style-scraper extract --url https://example.com --deps-dir .style-scraper/deps
style-scraper extract --url https://example.com --playwright-browser chromium
style-scraper extract --url https://example.com --no-install-deps
```

The default install policy is local to the workspace and does not make Node.js the runtime default.

## Error Contract

Structured errors are written to stderr:

```json
{
  "error": {
    "code": "BUN_NOT_FOUND",
    "message": "Bun runtime was not found. Install Bun or use the Docker image.",
    "phase": "capture",
    "recoverable": true
  }
}
```
