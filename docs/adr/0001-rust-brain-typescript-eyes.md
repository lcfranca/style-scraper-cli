# ADR-0001: Rust Brain + Bun/TypeScript Eyes

## Status

Accepted.

## Context

`style-scraper` must capture browser-rendered frontend facts and transform them into reproducible design-token and component-morphology payloads for AI agents.

The system has two radically different workloads:

1. Browser-native observation:
   - DOM inspection;
   - computed CSSOM extraction;
   - layout box extraction;
   - pseudo-state simulation;
   - accessibility snapshot capture;
   - screenshot capture.
2. Analytical processing:
   - token clustering;
   - color-space normalization;
   - graph analysis;
   - R-tree spatial indexing;
   - Gestalt grouping;
   - Atomic Design morphology inference;
   - W3C token emission;
   - deterministic schema validation.

A TypeScript-first architecture is idiomatic for browser automation, but it is not ideal as the core analytical processor for large DOM/CSSOM/layout graphs.

## Decision

Use Rust as the core CLI and analytical engine.

Use Bun + TypeScript + Playwright as an ephemeral browser probe invoked by Rust.

## Consequences

Positive:

- high-performance CPU-bound analysis;
- deterministic memory behavior;
- parallelizable clustering and graph algorithms;
- stable binary-centered CLI;
- strict ownership of final output contracts;
- clean separation between observation and inference.

Negative:

- more complex repository;
- cross-runtime packaging complexity;
- Bun and Playwright browser installation must be handled explicitly;
- IPC/schema contracts become mandatory.

## Rejected Alternatives

### TypeScript-first CLI

Rejected for production because it collapses browser I/O and heavy analysis into the same runtime.

### Rust-only browser automation

Rejected because Playwright's TypeScript ecosystem is the most idiomatic and operationally mature browser-control layer.

### Remote extraction service

Rejected because the tool must remain no-ops, local, ephemeral, and AI-agent-provider-agnostic.
