# ADR-0006: Security, Privacy, And Anti-Abuse Policy

## Status

Accepted.

## Decision

`style-scraper` is compliance-first, not stealth-first. It must not implement proxy rotation, CAPTCHA bypass, anti-bot evasion, credential collection, destructive interaction automation, or hidden exfiltration.

Text is hashed by default. Raw text requires `--include-text`. Redaction controls must be explicit for text, attributes, and images. Safe interactions allow non-destructive hover/focus state capture; clicks and form submissions are disabled by default.

## Crawl Safety

`--scope page` is the default. Crawl is opt-in, same-origin by default, capped by page/depth limits, and must stop or back off on 429, 403, 503, and `Retry-After`.

## Consequences

- authenticated pages require explicit `--auth-state`;
- sensitive capture risks are documented;
- rate-limit diagnostics are surfaced instead of evasion;
- dangerous routes such as logout, checkout, cart, delete, and admin are excluded by default in crawl mode.
