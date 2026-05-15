# ADR-0002: Bun Is The Default Probe Runtime

## Status

Accepted.

## Decision

The browser probe is executed with Bun by default. Node.js may be documented later as an explicit fallback target, but it is not the default runtime for the production architecture.

## Rationale

Bun gives fast TypeScript execution and a compact operational boundary for the ephemeral probe while keeping the Rust binary responsible for final orchestration and analysis.
