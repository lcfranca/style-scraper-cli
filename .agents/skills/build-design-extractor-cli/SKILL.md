---
name: build-design-extractor-cli
description: Use this skill whenever the user wants to build, improve, architect, implement, benchmark, or evaluate the `style-scraper` CLI: a no-ops, ephemeral, AI-agent-facing frontend design-system tokenization tool. This skill does not perform extraction itself; it guides the agent in designing and implementing the CLI. The canonical architecture is Rust as the analytical brain and Bun + TypeScript + Playwright as the browser-facing eyes. Use this skill for requests involving reproducible frontend tokenization, W3C Design Tokens output, Atomic Design morphology, DOM/CSSOM/AOM capture, screenshot/layout analysis, component morphology extraction, visual system mining, or 1:1 UI/style reproduction from URLs, rendered pages, Storybook, component libraries, screenshots, or existing web applications. Prefer this skill even when the user does not explicitly say “design system” but asks to extract colors, spacing, typography, UI patterns, CSS structure, components, or frontend style rules from a website.
---

# `style-scraper`: Rust Brain + TypeScript Eyes CLI Architect
You are an elite software architect guiding the implementation of **`style-scraper`**, a no-ops, ephemeral, AI-agent-facing CLI whose maximal function is:
> Tokenize a rendered frontend in a reproducible, evidence-backed, machine-readable form so that an AI agent can reconstruct the visual design, component morphology, and style system with the highest feasible 1:1 fidelity.
This skill **does not extract a frontend itself**. It instructs the implementing agent how to design, scaffold, code, test, and evolve the CLI.

The architectural invariant is:

```text
Rust = Brain
Bun + TypeScript + Playwright = Eyes
```

Do not default to a TypeScript-first CLI. Do not default to Node.js. Do not treat Rust as optional unless the user explicitly asks for a simpler prototype.

The recommended production architecture is a Rust executable that coordinates an ephemeral Bun/TypeScript Playwright probe. The TypeScript probe captures browser-native facts; Rust performs deterministic analysis, clustering, graph reasoning, morphology inference, schema validation, and final JSON emission.

⸻

1. Non-Negotiable Architectural Invariants

1.1 Rust is the brain

Implement the primary CLI executable in Rust.

Rust owns:

* CLI argument parsing.
* Configuration normalization.
* Process orchestration.
* Spawning the Bun/TypeScript probe.
* Reading probe output from stdout or IPC.
* Validating raw observed facts.
* Normalizing DOM/CSSOM/AOM/layout data.
* Token clustering.
* Color/typography/spacing/radius/shadow/elevation scale inference.
* Bounding-box and spatial topology computation.
* Atomic Design morphology classification.
* Accessibility and contrast analysis.
* W3C Design Tokens emission.
* Agent-facing JSON schema validation.
* Deterministic output sorting and hashing.
* Exit codes and error contracts.

Use Rust because the analytical core is CPU-bound, graph-heavy, memory-sensitive, and should be deterministic under large inputs.

Recommended Rust crates:

```text
clap              CLI parsing
serde             serialization/deserialization
serde_json        JSON I/O
schemars          JSON Schema generation
jsonschema        runtime schema validation when needed
tokio             async orchestration where useful
rayon             CPU-bound parallelism
anyhow/thiserror  error handling
tracing           internal structured diagnostics
tracing-subscriber diagnostics routing
indexmap          stable insertion-order maps
ordered-float     deterministic float handling
palette           color conversion and color spaces
deltae            perceptual color distance
rstar             R-tree spatial indexing
petgraph          graph reasoning over DOM/layout/component relations
uuid/ulid         stable artifact IDs when needed
sha2/blake3       stable hashes/fingerprints
```

1.2 Bun + TypeScript + Playwright are the eyes

Implement the browser probe in TypeScript, executed with Bun by default.

The probe owns:

* Launching Playwright.
* Navigating to the target URL.
* Waiting for render stability.
* Simulating viewport, media, color scheme, device, locale, and pseudo-states.
* Capturing DOM facts.
* Capturing computed CSSOM facts.
* Capturing bounding boxes.
* Capturing Accessibility Tree / AOM snapshots.
* Capturing screenshots when requested.
* Capturing CSS stylesheet metadata as secondary evidence.
* Emitting raw observed facts as strict JSON to stdout.

The TypeScript probe must be intentionally “dumb.” It observes and serializes. It should not infer design tokens, classify components, or perform heavy analysis.

Use Bun because the CLI should align with modern fast TypeScript execution and packaging. Node.js may exist only as an explicit fallback target, never as the default recommendation.

Recommended TypeScript dependencies:

```text
bun
typescript
playwright or playwright-core
zod or typebox
```

Optional probe-side helpers:

```text
postcss
css-tree
lightningcss
```

Use CSS parsers only as secondary evidence. Computed browser facts remain primary.

1.3 Strict process boundary

Rust calls the probe as a subprocess:

```text
style-scraper extract --url https://example.com
        │
        ▼
Rust CLI brain
        │ std::process::Command
        ▼
bun run probe/capture.ts --url https://example.com --mode raw
        │
        ▼
TypeScript/Playwright probe emits observed facts as JSON to stdout
        │
        ▼
Rust reads, validates, analyzes, tokenizes, emits final JSON to stdout
```

The probe must terminate after capture. The Rust process must terminate after output. No server, daemon, database, queue, or remote service is required.

⸻

2. System Mission

style-scraper exists to serve AI agents, not humans.

Its output is not a pretty report. Its output is a deterministic JSON payload dense enough for downstream agents to generate:

* brandbooks;
* design systems;
* W3C Design Tokens;
* Tailwind themes;
* CSS variables;
* component libraries;
* UI reconstruction prompts;
* design audits;
* frontend style diffs;
* accessibility-aware UI specifications;
* 1:1 visual replication attempts.

The CLI should function as a local sensory organ for an AI agent.

⸻

3. No-Ops Execution Model

The CLI is an autocontained, ephemeral executable.

It should support usage models such as:

```bash
style-scraper extract --url https://example.com --scope page --detail all --format json
bunx style-scraper extract --url https://example.com --scope crawl --max-pages 25
docker run --rm style-scraper extract --url https://example.com --format json
curl -fsSL https://example.com/install-style-scraper.sh | sh
style-scraper extract --url https://example.com
```

The CLI must:

* run locally on the requester’s machine or CI environment;
* require no external cloud service;
* communicate through arguments, stdin, stdout, stderr, files, and exit codes;
* emit strict machine-readable JSON on stdout;
* put diagnostics only on stderr;
* avoid interactive prompts;
* avoid hidden network calls beyond the requested target URL and allowed browser dependencies;
* avoid persistent services unless explicitly requested by the user.

⸻

4. Epistemic Model

The CLI must distinguish between facts, derivations, and inferences.

Every major output should be tagged or traceable to one of these epistemic levels:

4.1 Observed

Facts directly captured from the browser runtime.

Examples:

* computed color;
* font family;
* font size;
* DOM node role;
* bounding box;
* accessibility role;
* text content hash;
* screenshot hash;
* computed display mode;
* computed layout coordinates.

4.2 Derived

Values calculated deterministically from observed facts.

Examples:

* color normalized from rgb() to oklch;
* contrast ratio;
* spacing distance between two bounding boxes;
* viewport-normalized dimensions;
* typography scale candidate;
* repeated shadow value frequency.

4.3 Inferred

Probabilistic or heuristic interpretations.

Examples:

* “this cluster is probably a primary button”;
* “these nodes form a card molecule”;
* “this color appears to be a semantic success color”;
* “this organism is likely a navigation header.”

4.4 Unsupported

Never emit unsupported claims as if they were facts.

If the tool cannot prove or infer something with evidence, it should omit it or mark it as unresolved.

⸻

5. Theoretical Foundations

The CLI’s hypothesis of reproducibility is:

A rendered frontend can be approximated as a reproducible design system by capturing browser-computed visual facts, normalizing them into token scales, mapping spatial/component morphology, and emitting evidence-backed W3C-compatible token and component contracts.

This is not the claim that all frontend semantics can be perfectly recovered. The tool should be epistemically honest. It can support high-fidelity reconstruction by combining visual, structural, accessibility, and geometric evidence.

5.1 W3C Design Tokens: Absolute Value Theory

Use the W3C Design Tokens model as the canonical value layer.

Map raw observed values into:

* core/global tokens;
* semantic/alias tokens;
* component tokens;
* state tokens;
* responsive tokens;
* theme tokens where detected.

Token categories should include at minimum:

color
typography
fontFamily
fontSize
fontWeight
lineHeight
letterSpacing
spacing
sizing
radius
border
shadow
opacity
zIndex
motion
breakpoint
asset

Prefer W3C-style fields:

{
  "$type": "color",
  "$value": "#2563eb",
  "$description": "Dominant interactive blue observed in primary actions",
  "$extensions": {
    "styleScraper": {
      "evidence": ["node:btn-12", "node:cta-3"],
      "confidence": 0.92,
      "source": "computed-cssom"
    }
  }
}

Design-token emission must preserve traceability from token to observed facts.

5.2 Brad Frost Atomic Design: Morphological Structure

Use Atomic Design as the morphology layer.

Classify frontend units into:

atom
molecule
organism
template
page

Interpretation:

* Atom: indivisible visible or interactive primitive, such as button, input, label, icon, badge, text run.
* Molecule: small functional grouping, such as search field + button, form row, nav item, card header.
* Organism: larger section, such as navbar, hero, pricing card group, product grid, sidebar, footer.
* Template: page-level arrangement without specific content semantics.
* Page: rendered route or URL state.

Atomic classification must use evidence from:

* DOM ancestry;
* computed layout;
* repeated visual patterns;
* bounding-box proximity;
* accessibility roles;
* text density;
* interaction states;
* visual recurrence.

Do not equate DOM parenthood with Atomic Design structure. DOM is evidence, not truth.

5.3 Computed Inheritance Evaluation

Computed CSSOM is primary evidence.

Static CSS, class names, Tailwind utilities, CSS Modules, BEM names, inline styles, and stylesheet ASTs are secondary evidence.

Rationale:

* unknown websites may use any framework;
* static CSS may contain unused rules;
* utility classes may obscure visual semantics;
* JavaScript may mutate styles after load;
* browser computed styles are the closest accessible representation of rendered visual truth.

The probe should therefore capture computed styles for visible elements, not merely parse CSS files.

5.4 AOM / Accessibility Tree: Semantic Intent Layer

Capture the Accessibility Tree / AOM snapshot as semantic evidence.

Why:

* DOM and CSSOM say how something is painted;
* AOM indicates how the browser exposes user-facing roles and intentions;
* a visual button, link, tab, dialog, alert, or navigation landmark should be cross-checked against accessibility roles.

Use AOM to improve:

* atom classification;
* interactive role detection;
* control grouping;
* semantic component labeling;
* accessibility-aware reconstruction;
* false-positive reduction.

Example:

A visually button-like <div> without role="button" and without keyboard semantics should be classified differently from a proper <button>.

AOM is not perfect, but it is a high-value semantic signal.

5.5 Computational Gestalt: Spatial Perception Layer

Use computational Gestalt to infer visual grouping when DOM structure is misleading.

The CLI should reason over:

* proximity;
* similarity;
* alignment;
* containment;
* common region;
* continuity;
* symmetry;
* repetition;
* density;
* visual hierarchy.

Practical implementation:

* capture DOMRect for visible elements;
* index boxes using R-trees;
* compute nearest neighbors and overlap;
* cluster elements by spatial proximity;
* compare visual similarity vectors;
* infer molecule and organism candidates from geometric clusters;
* reconcile clusters against DOM and AOM evidence.

This is essential because CSS transforms, absolute positioning, grid, flex, portals, overlays, and framework abstractions often break the relation between DOM ancestry and perceived grouping.

5.6 Render Tree Pragmatism

Do not claim direct full access to an internal browser render tree unless the implementation truly has it.

Instead, approximate render-tree facts using:

* DOM;
* computed CSSOM;
* layout boxes;
* screenshots;
* accessibility snapshots;
* media queries;
* browser runtime evaluation.

Be precise in naming. Prefer “rendered layout snapshot” or “browser-observed visual facts” when the implementation is not directly consuming an internal render tree.

⸻

6. Repository Topology

Use this topology unless the user gives a stronger constraint:

style-scraper/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── crates/
│   ├── style-scraper-cli/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── args.rs
│   │   │   ├── exit_codes.rs
│   │   │   └── output.rs
│   │   └── Cargo.toml
│   │
│   ├── style-scraper-core/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── model/
│   │   │   │   ├── raw.rs
│   │   │   │   ├── tokens.rs
│   │   │   │   ├── morphology.rs
│   │   │   │   ├── evidence.rs
│   │   │   │   └── diagnostics.rs
│   │   │   ├── normalize/
│   │   │   │   ├── color.rs
│   │   │   │   ├── typography.rs
│   │   │   │   ├── spacing.rs
│   │   │   │   ├── shadow.rs
│   │   │   │   └── units.rs
│   │   │   ├── cluster/
│   │   │   │   ├── color_kmeans.rs
│   │   │   │   ├── spacing_scale.rs
│   │   │   │   └── typography_scale.rs
│   │   │   ├── geometry/
│   │   │   │   ├── rect.rs
│   │   │   │   ├── rtree.rs
│   │   │   │   ├── gestalt.rs
│   │   │   │   └── layout_graph.rs
│   │   │   ├── morphology/
│   │   │   │   ├── atomic.rs
│   │   │   │   ├── molecule.rs
│   │   │   │   ├── organism.rs
│   │   │   │   └── reconcile.rs
│   │   │   ├── a11y/
│   │   │   │   ├── contrast.rs
│   │   │   │   └── roles.rs
│   │   │   ├── emit/
│   │   │   │   ├── w3c_tokens.rs
│   │   │   │   ├── agent_json.rs
│   │   │   │   ├── css_vars.rs
│   │   │   │   └── tailwind.rs
│   │   │   └── validate/
│   │   │       ├── schema.rs
│   │   │       └── determinism.rs
│   │   └── Cargo.toml
│   │
│   └── style-scraper-probe-runner/
│       ├── src/
│       │   ├── lib.rs
│       │   ├── bun.rs
│       │   ├── command.rs
│       │   ├── ipc.rs
│       │   └── errors.rs
│       └── Cargo.toml
│
├── probe/
│   ├── package.json
│   ├── bun.lockb
│   ├── tsconfig.json
│   ├── src/
│   │   ├── capture.ts
│   │   ├── cli.ts
│   │   ├── schema.ts
│   │   ├── stability.ts
│   │   ├── dom.ts
│   │   ├── cssom.ts
│   │   ├── aom.ts
│   │   ├── layout.ts
│   │   ├── screenshots.ts
│   │   ├── states.ts
│   │   └── injected/
│   │       ├── collect-dom.ts
│   │       ├── collect-cssom.ts
│   │       ├── collect-layout.ts
│   │       └── collect-interactions.ts
│
├── schemas/
│   ├── raw-facts.schema.json
│   ├── style-scraper-output.schema.json
│   ├── w3c-tokens.schema.json
│   └── morphology.schema.json
│
├── fixtures/
│   ├── static-html/
│   ├── react-spa/
│   ├── tailwind/
│   ├── storybook/
│   └── complex-layout/
│
├── tests/
│   ├── integration/
│   ├── golden/
│   ├── determinism/
│   └── visual/
│
└── docs/
    ├── architecture.md
    ├── adr/
    │   ├── 0001-rust-brain-typescript-eyes.md
    │   ├── 0002-bun-not-node.md
    │   ├── 0003-computed-cssom-primary-evidence.md
    │   └── 0004-aom-gestalt-as-semantic-spatial-evidence.md
    └── cli.md

⸻

7. CLI Command Design

The executable name is:

`style-scraper`

7.1 Primary command

```bash
style-scraper extract --url https://example.com
```

This command performs capture + analysis + token emission.

Recommended flags:

```bash
style-scraper extract \
  --url https://example.com \
  --scope page \
  --detail all \
  --format json \
  --viewport 1440x900 \
  --color-scheme light \
  --states hover,focus-visible,active \
  --include-screenshots \
  --output -
```

7.2 Capture-only command

```bash
style-scraper capture --url https://example.com --raw-output raw-facts.json
```

Purpose:

* debug the TypeScript probe;
* store raw observed facts;
* support reproducibility;
* separate browser capture from Rust analysis.

This command should still be launched by Rust, not by asking the user to run the probe directly.

7.3 Analyze-only command

```bash
style-scraper analyze --input raw-facts.json --format json
```

Purpose:

* re-run Rust analysis without opening a browser;
* benchmark deterministic core behavior;
* support golden tests;
* avoid expensive repeated browsing.

7.4 Token-only command

```bash
style-scraper tokens --input raw-facts.json --format w3c
```

Purpose:

* emit W3C token dictionaries from previously captured facts.

7.5 Crawl command

```bash
style-scraper crawl --url https://example.com --max-pages 50 --same-origin
```

Purpose:

* discover internal routes;
* capture multiple frontend states;
* aggregate style system evidence across pages.

7.6 Diff command

```bash
style-scraper diff --before before.json --after after.json --format json
```

Purpose:

* compare two extractions;
* detect design drift;
* support CI checks.

7.7 Explain command

```bash
style-scraper explain --input output.json --token colors.semantic.primary
```

Purpose:

* explain why a token or component classification exists;
* return evidence references and confidence.

7.8 Validate command

```bash
style-scraper validate --input output.json
```

Purpose:

* validate output against JSON Schema;
* verify determinism metadata;
* fail CI on malformed output.

⸻

8. Scope Flags

The CLI must distinguish what to inspect.

--scope page

Only the provided URL.

--scope crawl

Crawl same-origin routes discovered from links, navigation, sitemap, or router hints.

--scope sitemap

Use sitemap URLs.

--scope routes-file

Use a user-provided file with routes.

--scope storybook

Inspect Storybook stories as component states.

--scope component-library

Inspect a component catalog if available.

Recommended options:

--max-pages 25
--same-origin
--include-path "/pricing"
--exclude-path "/blog/*"
--respect-robots
--no-respect-robots
--route-file routes.txt
--sitemap https://example.com/sitemap.xml

Default should be conservative:

--scope page

Do not crawl entire sites unless explicitly requested.

⸻

9. Detail Flags

The CLI must distinguish how deep the morphology should go.

Use:

--detail atomic
--detail molecule
--detail organism
--detail template
--detail page
--detail all

Interpretation:

--detail atomic

Extract atoms and primitive tokens.

Focus:

* text;
* buttons;
* inputs;
* labels;
* links;
* icons;
* badges;
* chips;
* images;
* primitive surfaces.

--detail molecule

Include small functional groups.

Focus:

* input + label;
* search box;
* nav item;
* card header;
* form field;
* menu item;
* button group.

--detail organism

Include large sections.

Focus:

* navbar;
* hero;
* footer;
* sidebar;
* pricing section;
* product grid;
* modal;
* checkout form;
* dashboard panel.

--detail template

Include layout-level structure independent of route content.

Focus:

* page scaffolds;
* grid;
* recurring layout slots;
* header/main/footer;
* sidebar/content regions.

--detail page

Include rendered route-level page state.

Focus:

* full page composition;
* route metadata;
* viewport state;
* page-level hierarchy.

--detail all

Emit all supported layers.

Default:

--detail all

But for very large crawls, recommend:

--detail organism

or

--detail molecule

to reduce payload size.

⸻

10. Output Modes

The primary output must be strict JSON on stdout.

Recommended formats:

--format json
--format w3c-tokens
--format raw
--format css-vars
--format tailwind
--format report-json

Rules:

* stdout is only machine-readable output.
* stderr is for diagnostics.
* No progress bars on stdout.
* No human prose unless --format human is explicitly requested.
* JSON object keys must be stable-sorted where possible.
* Floating point output must be rounded consistently.
* IDs must be deterministic across identical captures where possible.

⸻

11. Suggested Agent-Facing Output Schema

The final JSON should resemble:

{
  "schema_version": "0.1.0",
  "tool": {
    "name": "style-scraper",
    "version": "0.1.0"
  },
  "capture": {
    "url": "https://example.com",
    "scope": "page",
    "viewport": {
      "width": 1440,
      "height": 900,
      "device_scale_factor": 1
    },
    "color_scheme": "light",
    "timestamp": "2026-01-01T00:00:00Z",
    "content_hash": "sha256:..."
  },
  "evidence": {
    "observed_nodes": [],
    "observed_styles": [],
    "observed_layout": [],
    "observed_accessibility": [],
    "screenshots": []
  },
  "tokens": {
    "$description": "W3C-compatible token dictionary inferred from computed browser facts",
    "color": {},
    "typography": {},
    "spacing": {},
    "radius": {},
    "shadow": {},
    "border": {},
    "motion": {}
  },
  "morphology": {
    "atoms": [],
    "molecules": [],
    "organisms": [],
    "templates": [],
    "pages": []
  },
  "accessibility": {
    "roles": [],
    "contrast": [],
    "landmarks": []
  },
  "diagnostics": {
    "warnings": [],
    "unsupported": [],
    "confidence_summary": {}
  }
}

⸻

12. Raw Facts Schema

The Bun/TypeScript probe should emit raw facts like:

{
  "schema_version": "raw-facts.v1",
  "url": "https://example.com",
  "viewport": {
    "width": 1440,
    "height": 900,
    "device_scale_factor": 1
  },
  "pages": [
    {
      "url": "https://example.com",
      "title": "Example",
      "dom": {
        "nodes": [
          {
            "id": "node_1",
            "parent_id": null,
            "tag": "button",
            "attributes": {
              "class": "btn primary"
            },
            "text_hash": "sha256:...",
            "visible": true
          }
        ]
      },
      "cssom": {
        "computed_styles": [
          {
            "node_id": "node_1",
            "display": "inline-flex",
            "color": "rgb(255, 255, 255)",
            "background_color": "rgb(37, 99, 235)",
            "font_family": "Inter",
            "font_size": "14px",
            "font_weight": "600",
            "line_height": "20px",
            "padding": "8px 16px",
            "border_radius": "6px",
            "box_shadow": "none"
          }
        ]
      },
      "layout": {
        "boxes": [
          {
            "node_id": "node_1",
            "x": 120,
            "y": 240,
            "width": 96,
            "height": 36,
            "z_index": 1
          }
        ]
      },
      "accessibility": {
        "nodes": [
          {
            "node_id": "node_1",
            "role": "button",
            "name_hash": "sha256:...",
            "disabled": false,
            "focused": false
          }
        ]
      }
    }
  ]
}

Raw facts should avoid over-processing. Their job is observability.

⸻

13. W3C Design Token Output

Emit W3C-compatible tokens.

Example:

{
  "color": {
    "core": {
      "blue": {
        "500": {
          "$type": "color",
          "$value": "#2563eb",
          "$extensions": {
            "styleScraper": {
              "evidence": ["node_1", "node_8", "node_21"],
              "frequency": 12,
              "confidence": 0.95,
              "source": "computed-cssom"
            }
          }
        }
      }
    },
    "semantic": {
      "primary": {
        "$type": "color",
        "$value": "{color.core.blue.500}",
        "$extensions": {
          "styleScraper": {
            "inference": "dominant interactive action color",
            "evidence": ["component.button.primary", "component.nav.cta"],
            "confidence": 0.82
          }
        }
      }
    }
  }
}

Token inference must include:

* observed value;
* normalized value;
* source nodes;
* frequency;
* confidence;
* derivation path;
* semantic inference when applicable.

⸻

14. Atomic Morphology Output

Example:

{
  "type": "atom",
  "id": "atom.button.primary.1",
  "kind": "button",
  "signature": "button[role=button]",
  "evidence": {
    "dom_node_ids": ["node_1"],
    "aom_roles": ["button"],
    "layout_box_ids": ["box_1"],
    "computed_style_ids": ["style_1"]
  },
  "tokens": {
    "background_color": "{color.semantic.primary}",
    "text_color": "{color.core.neutral.0}",
    "radius": "{radius.core.6}",
    "padding_x": "{spacing.core.4}",
    "padding_y": "{spacing.core.2}"
  },
  "states": {
    "hover": {
      "background_color": "{color.core.blue.600}"
    },
    "focus-visible": {
      "outline": "{focus.ring.primary}"
    }
  },
  "confidence": 0.91
}

Molecule example:

{
  "type": "molecule",
  "id": "molecule.search_box.1",
  "kind": "search-box",
  "children": ["atom.input.search.1", "atom.button.submit.1"],
  "evidence": {
    "gestalt": {
      "proximity_score": 0.94,
      "alignment_score": 0.88,
      "common_region_score": 0.9
    },
    "aom_roles": ["searchbox", "button"],
    "dom_subtree_root": "node_40"
  },
  "confidence": 0.87
}

Organism example:

{
  "type": "organism",
  "id": "organism.header_nav.1",
  "kind": "header-navigation",
  "children": [
    "molecule.logo_lockup.1",
    "molecule.nav_links.1",
    "atom.button.primary.1"
  ],
  "evidence": {
    "landmark_role": "navigation",
    "layout_region": "top-horizontal-band",
    "recurrence": "present-on-3-pages"
  },
  "confidence": 0.89
}

⸻

15. Capture Strategy

The TypeScript probe should support:

* navigation timeout;
* render stability detection;
* network idle;
* animation settling;
* font loading readiness;
* viewport profiles;
* color scheme profiles;
* pseudo-state simulation;
* screenshot capture;
* accessibility snapshot;
* DOM serialization;
* computed style extraction;
* bounding-box extraction;
* route metadata.

Recommended stability approach:

1. Navigate.
2. Wait for DOMContentLoaded.
3. Wait for network idle or configurable timeout.
4. Wait for document.fonts.ready.
5. Observe layout mutations for a quiet window.
6. Disable or fast-forward animations if requested.
7. Capture facts.

Flags:

--wait domcontentloaded|load|networkidle|stable
--stability-window-ms 500
--timeout-ms 30000
--disable-animations
--wait-for-selector ".app"
--auth-state auth.json

⸻

16. State Extraction

Support relevant UI states:

--states hover,focus,focus-visible,active,disabled,visited

Implementation guidance:

* Use Playwright to hover/focus/click where safe.
* Prefer non-destructive interactions.
* Do not submit forms by default.
* Capture state deltas, not only absolute snapshots.
* Mark state evidence separately.

State output should say:

{
  "state": "hover",
  "target_node_id": "node_1",
  "changed_properties": {
    "background_color": {
      "before": "#2563eb",
      "after": "#1d4ed8"
    }
  }
}

⸻

17. Crawling Strategy

When --scope crawl is enabled:

* default to same-origin;
* respect max pages;
* deduplicate URLs;
* canonicalize query params;
* avoid destructive routes;
* avoid logout routes;
* avoid file downloads;
* optionally respect robots;
* allow include/exclude patterns;
* record route graph.

Commands:

```bash
style-scraper extract \
  --url https://example.com \
  --scope crawl \
  --max-pages 30 \
  --same-origin \
  --exclude-path "/logout" \
  --exclude-path "/admin/*"
```

Crawl output should preserve per-page evidence but aggregate global tokens.

⸻

18. Algorithmic Core in Rust

18.1 Color clustering

Use perceptual color spaces where possible.

Recommended pipeline:

1. Parse computed colors.
2. Normalize transparent values.
3. Convert to perceptual space such as OKLCH/LAB.
4. Remove near-duplicates.
5. Cluster by perceptual distance.
6. Preserve high-frequency exact values.
7. Infer core palettes.
8. Infer semantic colors through usage context.

Evidence features:

* frequency;
* element role;
* background/foreground relation;
* interactive usage;
* contrast relation;
* recurrence across pages;
* state transitions.

18.2 Typography scale inference

Inputs:

* font family;
* font size;
* font weight;
* line height;
* letter spacing;
* text role;
* bounding box;
* heading levels;
* AOM semantics.

Infer:

* display;
* heading;
* body;
* label;
* caption;
* code;
* button text.

Avoid relying purely on HTML tags. A <div> can visually function as a heading.

18.3 Spacing scale inference

Inputs:

* margins;
* padding;
* gaps;
* bounding-box distances;
* grid/flex gaps;
* repeated coordinate deltas.

Infer:

* core spacing scale;
* layout gap tokens;
* component padding tokens;
* section spacing tokens.

Use robust clustering to avoid every pixel delta becoming a token.

18.4 Radius, border, shadow, elevation

Extract:

* border radius;
* border width/style/color;
* box shadow;
* filter/drop shadow;
* opacity;
* backdrop effects;
* layer/elevation hints.

Cluster repeated values and preserve component-specific exceptional values.

18.5 Gestalt grouping

Use R-tree or equivalent spatial index.

Features:

* containment;
* overlap;
* nearest-neighbor distance;
* alignment;
* shared axis;
* visual similarity;
* common background region;
* whitespace boundaries;
* repeated pattern grids.

Use this to infer molecule and organism candidates.

18.6 Morphology reconciliation

Final component classification should reconcile:

DOM ancestry
+ AOM roles
+ layout geometry
+ computed style similarity
+ repeated patterns
+ text semantics
+ interaction states
+ screenshot/pixel evidence when available

Do not let one evidence source dominate unconditionally.

⸻

19. Determinism Requirements

The same input should produce the same output as much as possible.

Use:

* stable sorting;
* deterministic IDs;
* rounded floats;
* explicit viewport;
* pinned browser version where feasible;
* captured metadata;
* content hashes;
* schema versions;
* stable clustering seeds;
* golden snapshot tests.

Avoid:

* random IDs;
* timestamp-dependent IDs;
* nondeterministic map iteration;
* unbounded crawling;
* implicit viewport;
* hidden defaults that change output shape.

⸻

20. Error and Exit Contracts

Suggested exit codes:

```text
0  success
1  generic failure
2  invalid CLI arguments
3  browser/probe failure
4  navigation timeout
5  schema validation failure
6  analysis failure
7  output write failure
8  crawl policy violation
```

Errors on stderr should be structured JSON when requested:

--error-format json

Example:

{
  "error": {
    "code": "NAVIGATION_TIMEOUT",
    "message": "Timed out waiting for render stability",
    "phase": "capture",
    "url": "https://example.com",
    "recoverable": true
  }
}

Do not print errors to stdout.

⸻

21. Diagnostics

Diagnostics should include:

* unsupported CSS features;
* inaccessible pages;
* blocked resources;
* unstable layout;
* missing AOM data;
* contrast failures;
* low-confidence tokens;
* excessive DOM size;
* crawl truncation;
* screenshot capture failure;
* probe runtime mismatch.

Diagnostics must not contaminate the final agent JSON unless included in a structured diagnostics object.

⸻

22. Implementation Milestones

M0 — ADR and contracts

Create:

* architecture ADR;
* raw facts schema;
* final output schema;
* CLI command spec;
* error contract;
* determinism contract.

M1 — Rust CLI skeleton

Implement:

* style-scraper binary;
* clap arguments;
* output routing;
* exit codes;
* config normalization.

M2 — Bun/TypeScript probe skeleton

Implement:

* probe/src/capture.ts;
* Playwright launch;
* URL navigation;
* raw JSON stdout;
* basic schema validation.

M3 — Rust probe runner

Implement:

* Bun subprocess spawning;
* timeout handling;
* stdout/stderr separation;
* raw facts ingestion;
* error mapping.

M4 — Raw capture completeness

Capture:

* DOM;
* computed styles;
* layout boxes;
* AOM/accessibility snapshot;
* screenshots;
* viewport metadata;
* render stability.

M5 — Token engine

Implement:

* color clustering;
* typography inference;
* spacing inference;
* radius/border/shadow inference;
* W3C token emitter.

M6 — Atomic morphology

Implement:

* atoms;
* molecules;
* organisms;
* Gestalt grouping;
* AOM reconciliation;
* confidence scoring.

M7 — Crawl and aggregation

Implement:

* page vs crawl scope;
* route discovery;
* same-origin policy;
* route graph;
* cross-page token aggregation.

M8 — Diff and validation

Implement:

* output schema validation;
* token diff;
* design drift detection;
* deterministic golden tests.

M9 — Packaging

Implement:

* release binary;
* Bun probe bundling strategy;
* Docker image;
* bunx path if feasible;
* install script;
* CI pipeline.

⸻

23. Bun Probe Packaging Strategy

Because Rust is the brain but Playwright requires a browser automation ecosystem, the implementation must decide how to package the probe.

Acceptable strategies:

Strategy A — Development-first

Rust calls:

```bash
bun run probe/src/capture.ts
```

Best for development.

Strategy B — Bundled TypeScript artifact

Bundle probe to a single JS file using Bun:

```bash
bun build probe/src/capture.ts --target=bun --outfile=dist/probe.js
```

Rust calls:

```bash
bun dist/probe.js
```

Best for practical distribution where Bun is available.

Strategy C — Docker image

Package Rust binary + Bun + Playwright browsers into one container.

Best for CI and reproducibility.

Strategy D — Advanced embedded asset

Embed bundled probe JS into Rust binary and write it to a temporary directory at runtime.

Best for standalone distribution, but more complex because browser binaries and Bun runtime still need strategy.

Do not pretend that a Rust binary alone can magically include a full Playwright browser environment unless the packaging strategy actually handles it.

⸻

24. Security and Safety Boundaries

The CLI must not become a scraping abuse framework.

Default constraints:

* same-origin crawling unless overridden;
* no credential theft;
* no bypassing paywalls or access controls;
* no destructive actions;
* no automatic form submission;
* no hidden data exfiltration;
* no stealth mode by default;
* no CAPTCHA bypass;
* no evasion features.

For authenticated capture, require explicit user-provided browser state:

--auth-state auth.json

Never ask the agent to collect passwords.

⸻

25. Testing Strategy

Use fixture sites with known expected outputs.

Test categories:

25.1 Unit tests

Rust:

* color normalization;
* token clustering;
* geometry;
* R-tree grouping;
* morphology scoring;
* schema generation.

TypeScript:

* DOM capture;
* CSSOM capture;
* AOM capture;
* layout serialization.

25.2 Integration tests

Run:

style-scraper extract --url http://localhost:3000 --scope page --detail all

Against local fixture apps.

25.3 Golden snapshot tests

Store expected JSON outputs.

Require stable ordering and deterministic IDs.

25.4 Visual fixture tests

Use screenshot and known component layouts to verify Gestalt grouping.

25.5 Schema tests

Validate every output against JSON Schema.

25.6 Performance tests

Benchmark:

* 1k nodes;
* 10k nodes;
* 50k computed style entries;
* multi-page crawl aggregation;
* large screenshots when enabled.

Track:

* capture time;
* analysis time;
* peak memory;
* output size;
* token count for downstream AI consumption.

⸻

26. Example Commands

Extract one page at full detail:

```bash
style-scraper extract \
  --url https://example.com \
  --scope page \
  --detail all \
  --format json
```

Extract only atoms:

```bash
style-scraper extract \
  --url https://example.com \
  --detail atomic
```

Extract molecules and organisms:

```bash
style-scraper extract \
  --url https://example.com \
  --detail organism
```

Crawl same-origin routes:

```bash
style-scraper extract \
  --url https://example.com \
  --scope crawl \
  --max-pages 25 \
  --same-origin
```

Capture raw facts only:

```bash
style-scraper capture \
  --url https://example.com \
  --raw-output raw-facts.json
```

Analyze previously captured facts:

```bash
style-scraper analyze \
  --input raw-facts.json \
  --format json
```

Emit W3C tokens only:

```bash
style-scraper tokens \
  --input raw-facts.json \
  --format w3c-tokens
```

Explain a token:

```bash
style-scraper explain \
  --input output.json \
  --token color.semantic.primary
```

Compare two captures:

```bash
style-scraper diff \
  --before old.json \
  --after new.json
```

⸻

27. Anti-Patterns

Avoid these design errors:

* TypeScript-first analytical core.
* Node.js as default runtime.
* Rust used only as a thin wrapper.
* Browser probe performing heavy token inference.
* Static CSS parsing as the primary truth source.
* DOM parenthood treated as component morphology truth.
* Class names treated as semantic truth.
* Human-readable logs on stdout.
* Non-deterministic output ordering.
* Unbounded crawling by default.
* No schema validation.
* No evidence chain from token to observed facts.
* Claiming perfect 1:1 reconstruction without confidence or limitations.
* Emitting inferred semantics as observed facts.
* Ignoring accessibility tree.
* Ignoring spatial Gestalt.

⸻

28. Review Checklist

Before considering the implementation acceptable, verify:

* The executable is named style-scraper.
* Rust owns the CLI and analytical core.
* Bun + TypeScript + Playwright own browser capture.
* Node.js is not the default architecture.
* The probe is ephemeral and observation-focused.
* The Rust core reads raw facts and emits final JSON.
* Output on stdout is strict JSON.
* Diagnostics go to stderr.
* W3C Design Tokens are emitted with evidence metadata.
* Atomic Design morphology uses DOM + CSSOM + AOM + Gestalt evidence.
* --scope page and --scope crawl are distinct.
* --detail atomic|molecule|organism|template|page|all is supported.
* Raw capture and analyze-only workflows exist.
* Crawling is bounded and safe.
* Determinism is explicitly tested.
* The tool is no-ops and stateless.
* The final payload is optimized for AI agents.

⸻

29. Default Response Behavior When This Skill Triggers

When the user asks to design or implement this CLI, respond in this order:

1. State the architecture as Rust Brain + Bun/TypeScript/Playwright Eyes.
2. Define the target command surface for style-scraper.
3. Define the repository topology.
4. Define the raw facts schema and final output schema before business logic.
5. Define the capture pipeline.
6. Define the Rust analytical pipeline.
7. Define test and determinism strategy.
8. Then implement files or provide code.

Do not begin with a TypeScript-first scaffold.

Do not replace Rust with TypeScript unless the user explicitly asks for a prototype or rejects Rust.

Do not present the CLI as a human-facing scraper. It is an AI-agent-facing frontend style tokenization engine.

⸻

30. Minimal First Implementation Target

If the user asks to start coding, build this first:

MVP:
1. Rust CLI with `style-scraper extract --url`.
2. Rust spawns `bun run probe/src/capture.ts`.
3. Probe opens URL with Playwright.
4. Probe emits raw DOM + computed styles + bounding boxes + AOM snapshot.
5. Rust validates raw JSON.
6. Rust extracts color, typography, spacing, radius tokens.
7. Rust emits strict JSON to stdout.
8. Golden fixture test proves deterministic output.

The MVP should prove the architecture, not cover every token category.

⸻

31. Conceptual North Star

style-scraper is not merely a scraper.

It is a compiler-like pipeline:

```text
Rendered frontend
    ↓
Browser-observed facts
    ↓
Raw DOM/CSSOM/AOM/Layout/Screenshot evidence
    ↓
Rust normalization and graph analysis
    ↓
Design token inference
    ↓
Atomic morphology inference
    ↓
W3C-compatible, AI-agent-readable reconstruction payload
```

Its ideal output is not “what CSS exists on the page.”

Its ideal output is:

the smallest deterministic design-system representation sufficient for a downstream AI agent to reconstruct the frontend’s visual language with maximal fidelity and explicit evidence.

Continuação da skill:

## 32. Formal Architecture Decision Record
When documenting this CLI, create an ADR that explicitly rejects the TypeScript-first architecture for the production target.
Use this decision:
```markdown
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
Rejected because Playwright’s TypeScript ecosystem is the most idiomatic and operationally mature browser-control layer.
### Remote extraction service
Rejected because the tool must remain no-ops, local, ephemeral, and AI-agent-provider-agnostic.
```

⸻

33. Dataflow Contract

Implement the CLI as a staged compiler pipeline.

```text
User / AI Agent
    │
    │ style-scraper extract --url ...
    ▼
Rust CLI argument parser
    │
    ▼
Normalized RunConfig
    │
    ▼
Rust Probe Runner
    │
    │ spawns Bun subprocess
    ▼
Bun/TypeScript Playwright Probe
    │
    │ browser-observed facts
    ▼
RawFacts JSON
    │
    ▼
Rust RawFacts Validator
    │
    ▼
Normalization Layer
    │
    ├── color normalization
    ├── unit normalization
    ├── font normalization
    ├── rect normalization
    └── accessibility normalization
    │
    ▼
Analytical Core
    │
    ├── token clustering
    ├── graph construction
    ├── Gestalt grouping
    ├── Atomic Design classification
    ├── AOM reconciliation
    └── confidence scoring
    │
    ▼
Emitter Layer
    │
    ├── agent JSON
    ├── W3C Design Tokens
    ├── CSS variables
    ├── Tailwind config
    └── diff reports
    │
    ▼
stdout JSON
```

Every stage should be testable in isolation.

⸻

34. RawFacts Boundary

The TypeScript probe emits RawFacts.

RawFacts must not contain inferred design-system concepts.

Allowed in RawFacts:

* URL;
* viewport;
* browser metadata;
* DOM node facts;
* attributes;
* visibility;
* computed style values;
* bounding boxes;
* pseudo-state deltas;
* accessibility roles;
* screenshot references or hashes;
* stylesheet references;
* network/render stability metadata.

Not allowed in RawFacts:

* semantic token names like primary;
* Atomic Design labels like molecule;
* inferred component kinds like pricing-card;
* W3C token hierarchy;
* confidence scores for design-system meanings;
* final UI reconstruction advice.

Raw facts are observational evidence, not interpretation.

⸻

35. Final Output Boundary

The Rust core emits final structured knowledge.

Allowed in final output:

* normalized tokens;
* W3C token dictionary;
* semantic token aliases;
* component tokens;
* Atomic Design morphology;
* accessibility diagnostics;
* Gestalt clusters;
* route aggregation;
* confidence scores;
* evidence references;
* unresolved/unsupported facts;
* determinism metadata.

The Rust output is the only authoritative product of the CLI.

⸻

36. Confidence Model

Do not emit inferred classifications without confidence.

Use a numeric confidence scale:

0.00 - 0.39 = weak
0.40 - 0.69 = plausible
0.70 - 0.89 = strong
0.90 - 1.00 = near-certain

Confidence should be derived from evidence convergence.

Example:

{
  "classification": "primary-button",
  "confidence": 0.91,
  "evidence_weights": {
    "aom_role": 0.25,
    "visual_similarity": 0.2,
    "interactive_state": 0.2,
    "recurrence": 0.15,
    "dom_semantics": 0.1,
    "color_semantics": 0.1
  }
}

High confidence requires multiple independent signals.

A single class name like .btn-primary is not enough.

⸻

37. Evidence References

Every token, component, and inference should be traceable to evidence.

Use stable evidence IDs:

```text
page:<hash>
node:<stable-node-id>
style:<stable-style-id>
box:<stable-box-id>
aom:<stable-aom-id>
screenshot:<hash>
cluster:<stable-cluster-id>
```

Example:

{
  "$type": "dimension",
  "$value": "16px",
  "$extensions": {
    "styleScraper": {
      "evidence": [
        "style:button-primary-padding-x",
        "style:card-padding-x",
        "box:nav-item-gap"
      ],
      "frequency": 37,
      "confidence": 0.88
    }
  }
}

Evidence references let downstream agents audit why a design decision was inferred.

⸻

38. Token Naming Strategy

Token names must be deterministic and conservative.

38.1 Core token naming

Core tokens should be named by category and scale.

Examples:

color.core.blue.500
color.core.neutral.0
spacing.core.4
radius.core.2
typography.size.3
shadow.core.2

Avoid over-semantic names at the core level.

38.2 Semantic token naming

Semantic names require usage evidence.

Examples:

color.semantic.primary
color.semantic.surface
color.semantic.text.default
color.semantic.text.muted
color.semantic.danger
color.semantic.success
spacing.semantic.card.padding
radius.semantic.control
shadow.semantic.popover

Semantic aliases must point to core tokens where possible.

38.3 Component token naming

Component tokens should bind tokens to morphology.

Examples:

component.button.primary.background
component.button.primary.text
component.card.default.radius
component.input.default.border
component.navbar.default.height

38.4 Avoid false precision

Do not invent token names like brandRoyalDeepOceanUltraBlue unless the source explicitly provides such a name.

For unknown semantics, prefer:

color.semantic.interactive.1
color.semantic.accent.1
surface.elevated.1

over unsupported branding claims.

⸻

39. CSS Variable and Framework Mapping

The CLI may emit optional mappings to implementation targets.

CSS variables

```css
:root {
  --color-core-blue-500: #2563eb;
  --color-semantic-primary: var(--color-core-blue-500);
  --spacing-core-4: 16px;
  --radius-core-2: 6px;
}
```

Tailwind config

```javascript
export default {
  theme: {
    extend: {
      colors: {
        primary: "var(--color-semantic-primary)"
      },
      spacing: {
        "4": "var(--spacing-core-4)"
      },
      borderRadius: {
        control: "var(--radius-semantic-control)"
      }
    }
  }
}
```

These emitters are secondary. The canonical source remains the structured JSON and W3C tokens.

⸻

40. Handling Screenshots

Screenshots are optional but important for visual verification.

Support:

--include-screenshots
--screenshot full-page
--screenshot viewport
--screenshot-elements
--screenshot-dir artifacts/screenshots

Screenshot metadata should include:

{
  "id": "screenshot:page-main",
  "type": "viewport",
  "path": "artifacts/screenshots/page-main.png",
  "sha256": "...",
  "viewport": {
    "width": 1440,
    "height": 900
  }
}

Do not embed large base64 screenshots in stdout by default.

Use file paths and hashes unless the user explicitly requests inline artifacts.

⸻

41. Image and Pixel Analysis

Use pixel analysis as tertiary evidence.

Pixel evidence can help with:

* visual diff;
* screenshot regression;
* background region detection;
* shadow verification;
* anti-aliasing-aware visual comparison;
* checking whether computed CSSOM matches rendered pixels.

Rust may use image-processing crates for performance.

Potential crates:

image
imageproc
palette
rgb
fast_image_resize

Use pixel analysis carefully because screenshots include:

* anti-aliasing;
* font rendering differences;
* OS differences;
* browser version differences;
* device scale differences.

Do not treat pixel equality as universal truth unless environment is pinned.

⸻

42. Accessibility-Aware Reconstruction

The output should help downstream agents reconstruct not only appearance but accessible UI structure.

Capture and emit:

* role;
* name hash or redacted accessible name;
* state;
* disabled;
* checked;
* expanded;
* selected;
* focused;
* landmark;
* heading level;
* form relationships where available.

Example:

{
  "id": "atom.button.primary.1",
  "kind": "button",
  "a11y": {
    "role": "button",
    "name_hash": "sha256:...",
    "states": {
      "disabled": false,
      "focused": false
    }
  }
}

Do not leak sensitive text by default. Prefer hashes or configurable redaction.

⸻

43. Privacy and Redaction

The CLI may inspect real websites or authenticated pages.

Provide redaction controls:

--redact-text
--redact-attributes
--redact-images
--hash-text
--include-text

Default recommendation:

hash or redact text content unless explicit content extraction is required

The design-system task usually needs typography, layout, roles, and style values, not raw private content.

Preserve:

* text length;
* approximate line count;
* typographic role;
* bounding box;
* hash;
* semantic role.

Avoid exposing:

* emails;
* names;
* addresses;
* tokens;
* secrets;
* user content;
* hidden form values.

⸻

44. Authentication

If the user needs authenticated pages, support:

--auth-state auth.json

This should load a Playwright storage state file.

Do not implement password collection flows as the default.

Optional:

```bash
style-scraper auth login --url https://example.com --output auth.json
```

If implemented, this should open a controlled browser session for the user to log in manually, then save storage state. Do not ask the AI agent to handle credentials.

⸻

45. Browser Profiles

Support explicit browser and device profiles:

--browser chromium
--browser firefox
--browser webkit
--viewport 1440x900
--device "iPhone 15"
--device-scale-factor 2
--color-scheme light
--color-scheme dark
--reduced-motion reduce
--locale pt-BR
--timezone America/Sao_Paulo

Default:

chromium
1440x900
light
no forced locale unless specified

Cross-browser extraction should be opt-in because it multiplies output and runtime.

⸻

46. Responsive Extraction

Support multiple viewport captures:

--viewports 375x812,768x1024,1440x900

Output should group tokens by:

* global invariant;
* viewport-specific variation;
* breakpoint candidate;
* component layout variant.

Example:

{
  "component": "organism.header_nav.1",
  "responsive": {
    "375x812": {
      "layout": "collapsed",
      "tokens": {
        "height": "{sizing.header.mobile}"
      }
    },
    "1440x900": {
      "layout": "horizontal",
      "tokens": {
        "height": "{sizing.header.desktop}"
      }
    }
  }
}

Breakpoint inference must be evidence-backed.

⸻

47. Storybook Mode

Storybook is a high-value source because it exposes component states.

Support:

--scope storybook
--storybook-url http://localhost:6006

Capture:

* stories;
* controls where possible;
* variants;
* component states;
* themes;
* viewport permutations.

Storybook mode should prioritize component-level morphology over route-level page morphology.

⸻

48. Component Library Mode

For design systems with component catalogs, support:

--scope component-library

The CLI may inspect:

* Storybook;
* Ladle;
* Histoire;
* documentation pages;
* component playgrounds;
* static catalogs.

Do not assume a specific framework.

⸻

49. Framework-Agnosticism

The CLI must not depend on React, Vue, Angular, Svelte, Solid, Astro, Next.js, Remix, Tailwind, Bootstrap, or any framework as the primary extraction model.

Framework-specific hints are allowed only as secondary evidence.

Primary evidence remains:

browser-observed DOM + computed CSSOM + layout + AOM + screenshot metadata

⸻

50. Handling Tailwind and Utility CSS

Tailwind class names can be useful but must not be the source of truth.

A Tailwind site should be handled by computed styles even if:

* class names are minified;
* classes are generated;
* CSS is purged;
* arbitrary values are used;
* class names are absent in runtime;
* styles are injected dynamically.

Optional Tailwind emitter can be generated from inferred tokens, but the tool should not need Tailwind to work.

⸻

51. Handling CSS-in-JS

CSS-in-JS systems may inject dynamic styles.

The probe should capture:

* computed styles;
* style tags;
* CSSStyleSheet metadata where accessible;
* adoptedStyleSheets where accessible;
* shadow DOM styles where accessible.

Do not require source maps.

⸻

52. Shadow DOM

Support open Shadow DOM when accessible.

Capture:

* shadow root nodes;
* computed styles;
* layout boxes;
* component boundaries;
* slots.

Closed Shadow DOM cannot be inspected directly. Mark it as unsupported or partially observable through layout and screenshot evidence.

⸻

53. Iframes

Support iframes cautiously.

Capture same-origin iframes when allowed.

For cross-origin iframes:

* record bounding box;
* record URL if accessible;
* record limitation diagnostic;
* do not bypass browser security.

⸻

54. Animation and Motion Tokens

Motion tokens should be extracted from computed styles and CSS rules where possible.

Capture:

* transition duration;
* transition timing function;
* transition property;
* animation duration;
* animation timing function;
* animation name;
* delay;
* reduced-motion behavior.

Emit:

{
  "motion": {
    "duration": {
      "fast": {
        "$type": "duration",
        "$value": "150ms"
      }
    },
    "easing": {
      "standard": {
        "$type": "cubicBezier",
        "$value": [0.4, 0, 0.2, 1]
      }
    }
  }
}

Motion extraction should distinguish observed computed values from inferred semantic categories.

⸻

55. Z-Index and Layering

Extract stacking/layer information:

* z-index;
* position;
* transform-induced stacking contexts;
* opacity-induced stacking contexts;
* fixed/sticky overlays;
* dialog/popover layers.

Emit layer tokens where recurrent:

zIndex.dropdown
zIndex.modal
zIndex.toast
zIndex.stickyHeader

Use caution: semantic names require evidence such as AOM role, position, and recurrence.

⸻

56. Theming

Support theme captures:

--themes light,dark

or:

--color-scheme light
--color-scheme dark

Output should distinguish:

* invariant core values;
* theme-specific aliases;
* semantic tokens with mode-specific values.

Example:

{
  "color": {
    "semantic": {
      "surface": {
        "$type": "color",
        "$value": {
          "light": "{color.core.neutral.0}",
          "dark": "{color.core.neutral.950}"
        }
      }
    }
  }
}

⸻

57. Agent Payload Compression

Because the output is consumed by AI agents, manage payload size.

Provide:

--compact
--pretty
--max-nodes
--max-evidence-per-token
--include-raw-evidence
--no-raw-evidence
--evidence-level full|summary|minimal

Default should preserve enough evidence for audit while avoiding excessive raw DOM dumps.

For large sites, emit:

* token dictionary;
* morphology summaries;
* evidence references;
* optional external artifact files.

⸻

58. Artifact Directory

Support:

--artifact-dir .style-scraper/artifacts

Use artifact directory for:

* raw facts;
* screenshots;
* debug traces;
* intermediate graphs;
* large evidence files;
* schemas;
* crawl route maps.

The final stdout can reference artifact paths and hashes.

⸻

59. Reproducibility Manifest

Each run should emit a manifest.

Example:

{
  "reproducibility": {
    "style_scraper_version": "0.1.0",
    "rust_target": "aarch64-apple-darwin",
    "probe_runtime": "bun",
    "bun_version": "1.x",
    "playwright_version": "1.x",
    "browser": "chromium",
    "browser_version": "x.y.z",
    "viewport": "1440x900",
    "device_scale_factor": 1,
    "color_scheme": "light",
    "input_url": "https://example.com",
    "capture_hash": "sha256:...",
    "analysis_hash": "sha256:..."
  }
}

This lets downstream agents and CI compare outputs over time.

⸻

60. CI/CD Use Case

The CLI should support design drift detection.

Example:

style-scraper extract --url http://localhost:3000 --output current.json
style-scraper diff --before baseline.json --after current.json --fail-on major

Diff severity:

none
minor
moderate
major
breaking

Diff dimensions:

* token changes;
* component morphology changes;
* accessibility regression;
* layout shifts;
* contrast changes;
* route/component disappearance.

⸻

61. Suggested Rust Module Interfaces

Use interfaces that preserve separation.

Example conceptual Rust interfaces:

pub trait TokenExtractor {
    fn extract(&self, facts: &NormalizedFacts) -> anyhow::Result<TokenSet>;
}
pub trait MorphologyClassifier {
    fn classify(&self, facts: &NormalizedFacts, tokens: &TokenSet) -> anyhow::Result<MorphologyGraph>;
}
pub trait Emitter {
    fn emit(&self, model: &StyleScraperModel) -> anyhow::Result<String>;
}

Probe runner boundary:

pub trait ProbeRunner {
    fn capture(&self, config: &CaptureConfig) -> anyhow::Result<RawFacts>;
}

Keep browser process orchestration out of token algorithms.

⸻

62. Suggested TypeScript Probe Interfaces

The TypeScript probe should be schema-first.

Example:

type CaptureRequest = {
  url: string;
  viewport: {
    width: number;
    height: number;
    deviceScaleFactor?: number;
  };
  colorScheme?: "light" | "dark";
  states?: Array<"hover" | "focus" | "focus-visible" | "active">;
  includeScreenshots?: boolean;
  waitMode?: "domcontentloaded" | "load" | "networkidle" | "stable";
};
type RawFacts = {
  schema_version: "raw-facts.v1";
  url: string;
  viewport: ViewportFacts;
  pages: PageFacts[];
  diagnostics: ProbeDiagnostic[];
};

Validate probe output before printing.

The probe should fail loudly if it cannot produce valid JSON.

⸻

63. IPC Strategy

Default IPC:

Rust passes args to Bun subprocess.
Bun emits RawFacts JSON to stdout.
Rust reads stdout fully.
Bun diagnostics go to stderr.

For very large captures, support file-based IPC:

--raw-output /tmp/raw-facts.json

or internal temp file exchange:

Rust creates temp file path.
Rust passes path to Bun.
Bun writes raw facts file.
Rust reads file.
Rust deletes temp file unless --keep-artifacts.

Avoid streaming partial JSON unless needed. Streaming complicates validation.

⸻

64. Handling Large Sites

For large crawls:

* process page captures incrementally;
* aggregate token candidates progressively;
* cap raw evidence;
* store artifacts externally;
* parallelize Rust analysis;
* rate-limit browser navigation;
* enforce max pages;
* enforce max DOM nodes per page;
* warn on truncation.

Suggested flags:

--max-pages 100
--max-nodes-per-page 15000
--max-output-mb 50
--parallel-pages 3
--analysis-threads auto

Rust should own analysis parallelism. The TypeScript probe should avoid uncontrolled concurrency.

⸻

65. Performance Philosophy

Performance should be measured by phase:

capture_time_ms
probe_serialization_time_ms
raw_json_size_bytes
rust_parse_time_ms
normalization_time_ms
token_inference_time_ms
morphology_time_ms
emit_time_ms
peak_memory_mb

Emit performance metadata when requested:

--include-performance

Do not optimize blindly. Benchmark the pipeline.

⸻

66. Local Development Commands

Suggested development commands:

cargo build
cargo test
cargo run -p style-scraper-cli -- extract --url https://example.com
cd probe
bun install
bun run src/capture.ts --url https://example.com
cargo run -p style-scraper-cli -- capture --url https://example.com --raw-output fixtures/out/raw.json
cargo run -p style-scraper-cli -- analyze --input fixtures/out/raw.json --output fixtures/out/final.json

⸻

67. Minimum Files to Generate First

When asked to scaffold the project, generate these first:

Cargo.toml
crates/style-scraper-cli/Cargo.toml
crates/style-scraper-cli/src/main.rs
crates/style-scraper-cli/src/args.rs
crates/style-scraper-core/Cargo.toml
crates/style-scraper-core/src/lib.rs
crates/style-scraper-core/src/model/raw.rs
crates/style-scraper-core/src/model/tokens.rs
crates/style-scraper-probe-runner/Cargo.toml
crates/style-scraper-probe-runner/src/lib.rs
probe/package.json
probe/tsconfig.json
probe/src/capture.ts
probe/src/schema.ts
schemas/raw-facts.schema.json
schemas/style-scraper-output.schema.json
docs/adr/0001-rust-brain-typescript-eyes.md

Do not start with a pure TypeScript package as the root.

⸻

68. Minimal Rust CLI Skeleton

The first CLI skeleton should roughly follow:

use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "style-scraper")]
#[command(about = "AI-agent-facing frontend design tokenization CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    Extract {
        #[arg(long)]
        url: String,
        #[arg(long, default_value = "page")]
        scope: String,
        #[arg(long, default_value = "all")]
        detail: String,
        #[arg(long, default_value = "json")]
        format: String,
    },
    Capture {
        #[arg(long)]
        url: String,
        #[arg(long)]
        raw_output: Option<String>,
    },
    Analyze {
        #[arg(long)]
        input: String,
    },
    Tokens {
        #[arg(long)]
        input: String,
        #[arg(long, default_value = "w3c-tokens")]
        format: String,
    },
    Diff {
        #[arg(long)]
        before: String,
        #[arg(long)]
        after: String,
    },
    Validate {
        #[arg(long)]
        input: String,
    },
}

Then wire commands to modules. Keep main.rs thin.

⸻

69. Minimal Bun Probe Skeleton

The first probe should roughly follow:

import { chromium } from "playwright";
import { z } from "zod";
const ArgsSchema = z.object({
  url: z.string().url()
});
async function main() {
  const url = parseUrlFromArgs();
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({
    viewport: { width: 1440, height: 900 },
    colorScheme: "light"
  });
  await page.goto(url, { waitUntil: "domcontentloaded" });
  await page.waitForLoadState("networkidle").catch(() => undefined);
  await page.evaluate(() => document.fonts?.ready);
  const facts = await page.evaluate(() => {
    const nodes = Array.from(document.querySelectorAll("*"))
      .map((el, index) => {
        const rect = el.getBoundingClientRect();
        const style = getComputedStyle(el);
        return {
          id: `node_${index}`,
          tag: el.tagName.toLowerCase(),
          visible: rect.width > 0 && rect.height > 0 && style.visibility !== "hidden",
          attributes: {
            role: el.getAttribute("role"),
            class: el.getAttribute("class"),
            id: el.getAttribute("id")
          },
          layout: {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height
          },
          computed_style: {
            display: style.display,
            position: style.position,
            color: style.color,
            backgroundColor: style.backgroundColor,
            fontFamily: style.fontFamily,
            fontSize: style.fontSize,
            fontWeight: style.fontWeight,
            lineHeight: style.lineHeight,
            letterSpacing: style.letterSpacing,
            padding: style.padding,
            margin: style.margin,
            borderRadius: style.borderRadius,
            boxShadow: style.boxShadow,
            zIndex: style.zIndex
          }
        };
      });
    return {
      schema_version: "raw-facts.v1",
      url: location.href,
      title: document.title,
      nodes
    };
  });
  const accessibility = await page.accessibility.snapshot({
    interestingOnly: false
  }).catch(() => null);
  await browser.close();
  process.stdout.write(JSON.stringify({
    ...facts,
    accessibility
  }));
}
main().catch((error) => {
  process.stderr.write(JSON.stringify({
    error: {
      code: "PROBE_FAILURE",
      message: String(error?.message ?? error)
    }
  }));
  process.exit(3);
});

This is only an MVP probe. The production probe must modularize DOM, CSSOM, AOM, layout, state, and screenshot capture.

⸻

70. When User Asks “Is Rust Worth It?”

Answer:

Rust is worth it for the production target because style-scraper is not just a Playwright wrapper. It is a compiler-like analytical engine over a large visual graph.

Use TypeScript for what requires browser idiom:

* Playwright;
* DOM runtime;
* CSSOM runtime;
* AOM snapshot;
* pseudo-state simulation.

Use Rust for what requires deterministic computation:

* clustering;
* graph analysis;
* spatial indexing;
* large JSON validation/normalization;
* morphology inference;
* evidence graph construction;
* output determinism;
* packaging as a stable CLI.

The hybrid architecture has more complexity, but the complexity aligns with the true boundary between observation and analysis.

⸻

71. When User Asks for a Simpler Prototype

If the user explicitly requests speed over architecture, allow a temporary prototype:

Prototype: Bun + TypeScript only.
Production: Rust Brain + Bun/TypeScript Eyes.

Make clear that TypeScript-only is acceptable for proof-of-concept, not the gold-standard target.

The skill’s default should remain Rust Brain + TypeScript Eyes.

⸻

72. When User Asks for Implementation Order

Recommend this exact order:

1. Define schemas.
2. Build Bun probe MVP.
3. Build Rust CLI shell.
4. Wire Rust to spawn Bun.
5. Validate RawFacts in Rust.
6. Emit minimal normalized output.
7. Add token extraction.
8. Add W3C emitter.
9. Add AOM capture.
10. Add bounding-box Gestalt grouping.
11. Add Atomic Design morphology.
12. Add crawl mode.
13. Add diff mode.
14. Add packaging and deterministic tests.

Reason: schemas stabilize the boundary before algorithms become complex.

⸻

73. When User Asks for Architecture Diagram

Use this Mermaid diagram:

flowchart TD
    A[AI Agent / User] --> B[style-scraper Rust CLI]
    B --> C[RunConfig Normalizer]
    C --> D[Probe Runner]
    D --> E[Bun + TypeScript Playwright Probe]
    E --> F[Browser Runtime]
    F --> G[DOM]
    F --> H[Computed CSSOM]
    F --> I[Layout Boxes]
    F --> J[AOM Snapshot]
    F --> K[Screenshots]
    G --> L[RawFacts JSON]
    H --> L
    I --> L
    J --> L
    K --> L
    L --> M[Rust RawFacts Validator]
    M --> N[Normalization Layer]
    N --> O[Token Engine]
    N --> P[Gestalt Geometry Engine]
    N --> Q[AOM Semantic Engine]
    O --> R[Morphology Classifier]
    P --> R
    Q --> R
    R --> S[W3C Tokens + Morphology Graph]
    S --> T[Strict JSON stdout]

⸻

74. When User Asks for Command Examples

Give examples grouped by intent.

One page, full extraction

style-scraper extract --url https://example.com --scope page --detail all

Full crawl, bounded

style-scraper extract \
  --url https://example.com \
  --scope crawl \
  --max-pages 30 \
  --same-origin

Only atoms

style-scraper extract --url https://example.com --detail atomic

Molecules and organisms

style-scraper extract --url https://example.com --detail organism

Responsive design

style-scraper extract \
  --url https://example.com \
  --viewports 375x812,768x1024,1440x900

Dark and light theme

style-scraper extract \
  --url https://example.com \
  --themes light,dark

Raw capture for debugging

style-scraper capture --url https://example.com --raw-output raw.json

Analyze without browser

style-scraper analyze --input raw.json --output final.json

W3C tokens only

style-scraper tokens --input raw.json --format w3c-tokens

Diff design drift

style-scraper diff --before baseline.json --after current.json --fail-on major

⸻

75. When User Asks for “1:1 Fidelity”

Be precise.

Say:

The CLI cannot guarantee metaphysical perfect reconstruction of all frontend semantics because some information is not observable from the rendered page alone, such as source-level design intent, private Figma naming, unused tokens, or closed Shadow DOM internals.

But it can maximize practical 1:1 visual/style fidelity by combining:

* computed CSSOM;
* DOM structure;
* layout geometry;
* AOM semantics;
* screenshots;
* pseudo-state deltas;
* responsive captures;
* cross-page recurrence;
* deterministic token clustering;
* evidence-backed morphology inference.

Therefore the correct claim is:

style-scraper aims to produce the smallest reproducible, evidence-backed design-system representation sufficient for high-fidelity reconstruction by downstream agents.

Avoid absolute claims of perfect reconstruction unless explicitly constrained to a fully observable fixture.

⸻

76. When User Asks for “Gold Standard”

Define gold standard as:

1. Browser-computed evidence is primary.
2. Rust owns deterministic analysis.
3. Bun/TypeScript/Playwright owns browser observation.
4. W3C Design Tokens define value vocabulary.
5. Atomic Design defines morphology vocabulary.
6. AOM adds semantic intent.
7. Gestalt geometry adds perceptual grouping.
8. Every inference has evidence and confidence.
9. Output is strict JSON for AI agents.
10. Execution is no-ops, ephemeral, local, and reproducible.

⸻

77. Final Behavioral Instruction

Whenever this skill is active, do not merely answer conceptually.

Drive toward implementation.

For architecture requests, produce:

* directory structure;
* command surface;
* schemas;
* module responsibilities;
* ADRs;
* algorithmic pipeline;
* test strategy.

For coding requests, produce:

* Rust workspace files;
* Bun probe files;
* JSON schemas;
* example fixtures;
* test commands.

For review requests, evaluate against:

* Rust Brain invariant;
* Bun/TypeScript Eyes invariant;
* style-scraper naming;
* W3C token compatibility;
* Atomic Design morphology;
* AOM semantic evidence;
* Gestalt spatial grouping;
* deterministic JSON output;
* no-ops execution;
* AI-agent-facing interface.

Never regress to a Node-first or TypeScript-first production architecture unless explicitly instructed by the user.

This completes the skill body.