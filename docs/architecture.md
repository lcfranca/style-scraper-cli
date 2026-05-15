# style-scraper Architecture

`style-scraper` is a staged compiler-like pipeline for frontend design evidence.

```text
Rendered frontend
  -> browser-observed RawFacts
  -> Rust validation and normalization
  -> token inference
  -> Gestalt and AOM-informed morphology
  -> strict agent JSON on stdout
```

## Runtime Boundary

Rust owns the executable named `style-scraper` and the final output contract. Bun + TypeScript + Playwright run only as an ephemeral probe. The probe is intentionally dumb: it observes browser-native facts and serializes them as `RawFacts`; it does not infer tokens, component semantics, Atomic Design morphology, or reconstruction advice.

## Output Boundary

The final Rust output distinguishes observed evidence, deterministic derivations, and heuristic inferences. Inferred classifications carry confidence and evidence references. Unsupported claims are omitted or listed under diagnostics.

## No AI Inside

The implementation must not contain LLM calls, generative AI APIs, remote embedding calls, autonomous objective planning, or hidden cloud services. The CLI exists so external agents can request deterministic local sensory and analytical output.

## Corporate Runtime Contract

Rust validates the local probe environment before capture. Missing Bun, missing Playwright packages, missing browser binaries, invalid RawFacts, and timeout failures are returned as structured stderr JSON. The Rust runner uses temporary file IPC for RawFacts to avoid large stdout pipe pressure, then deletes temporary files unless `--keep-artifacts` is set.

## Safety Posture

`style-scraper` is compliance-first. Crawl is opt-in, page scope is default, text is hashed by default, raw text is opt-in, screenshots are optional, and no stealth, proxy rotation, CAPTCHA bypass, credential collection, form submission, or destructive click automation is implemented.
