# ADR-0005: No-Ops Ephemeral Execution

## Status

Accepted.

## Decision

`style-scraper` runs as an on-demand local CLI. It does not start a daemon, server, queue, database, background worker, telemetry service, or remote extraction service. The Rust process invokes an ephemeral Bun/TypeScript probe and exits after emitting strict JSON or a structured error.

## Consequences

- stdout remains reserved for machine-readable payloads;
- stderr carries diagnostics and structured errors;
- temporary capture artifacts are deleted unless `--keep-artifacts` is requested;
- all crawl expansion must be explicit and bounded;
- browser access is limited to the requested target and user-provided auth state.
