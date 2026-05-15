# ADR-0004: AOM And Gestalt Inform Morphology

## Status

Accepted.

## Decision

Atomic Design morphology is inferred from a reconciliation of DOM ancestry, computed styles, layout geometry, Accessibility Tree evidence, and computational Gestalt grouping.

## Rationale

DOM parenthood alone is not a reliable proxy for perceived component structure. AOM adds semantic intent, while layout geometry captures visual grouping even when portals, grids, flexbox, transforms, or absolute positioning distort the DOM-to-screen relationship.
