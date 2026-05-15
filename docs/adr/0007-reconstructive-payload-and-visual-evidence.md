# ADR 0007: Reconstructive Payload and Visual Evidence

## Status

Accepted

## Context

DOM, computed CSSOM, layout boxes, AOM roles, tokens, Gestalt clusters, and Atomic Design morphology are enough for an inventory of rendered facts. They are not enough, by themselves, to claim a 1:1 reconstruction of a frontend.

Pixel evidence and visual comparison are required because many design details are browser-native, asset-driven, pseudo-element-driven, responsive, or perceptual.

## Decision

`style-scraper` will emit a reconstruction-oriented model that stays evidence-first and deterministic:

- screenshots are stored as artifacts and referenced by path plus SHA-256;
- pseudo-elements, assets, CSS variables, state deltas, and layout constraints are explicit evidence;
- tokens are bound to atoms and reconstruction components when a deterministic token match exists;
- large centering offsets are represented as layout constraints, not reusable spacing tokens;
- `--format reconstruction-json` emits component specs and hints, not generated HTML/CSS;
- `diff-visual` provides deterministic pixel-diff fidelity evidence.

## Consequences

The CLI remains a sensory and analytical provider for external agents. It still does not contain LLMs, generative AI, embeddings, autonomous agents, or business-goal decision logic. A 1:1 reconstruction claim must be made by a caller only after comparing screenshots and inspecting the visual fidelity payload.
