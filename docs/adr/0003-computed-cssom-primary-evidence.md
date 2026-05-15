# ADR-0003: Computed CSSOM Is Primary Evidence

## Status

Accepted.

## Decision

Computed browser styles are the primary style evidence for token inference. Static CSS, class names, framework hints, CSS-in-JS artifacts, and stylesheet ASTs are secondary evidence only.

## Rationale

The rendered browser state is closest to visual truth. Static source data can include unused rules, generated classes, dynamic injection, framework-specific indirection, and stale declarations.
