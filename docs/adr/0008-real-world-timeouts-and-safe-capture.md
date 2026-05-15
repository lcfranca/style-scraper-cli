# ADR 0008: Real-World Timeouts and Safe Capture

## Status

Accepted

## Context

Production websites often keep network, timers, and DOM mutations alive after the useful page has rendered. Waiting strictly for `networkidle` or a perfect stability window can turn a valid capture into a false `NAVIGATION_TIMEOUT`.

Some sites also expose rate limits, WAF/challenge pages, or CAPTCHA flows. The CLI must diagnose those conditions without attempting evasion.

## Decision

`style-scraper` uses `--wait auto` by default. The probe performs progressive wait phases:

1. initial `DOMContentLoaded` navigation;
2. short `load` wait;
3. short `document.fonts.ready` wait;
4. short `networkidle` wait, treated as non-fatal by default;
5. bounded mutation stability window;
6. visible DOM check and capture-on-timeout when useful content exists.

The Rust runner now uses a subprocess watchdog separate from browser-phase timeouts. Probe progress is written to `stderr`, while `stdout` remains strict JSON.

`--safe-capture` enables a conservative, compliance-first capture profile: page scope, safe interactions, no click/form submission, download blocking, and resource budgets. It does not implement stealth scraping, proxy rotation, fingerprint spoofing, CAPTCHA solving, WAF bypass, or other evasion.

## Consequences

Timeouts in non-critical phases become warnings in `diagnostics` when visible DOM was captured. Fatal timeout remains possible only when initial navigation produces no capturable content or the subprocess watchdog fires without recoverable RawFacts.
