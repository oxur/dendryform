---
number: 1
title: "dendryform — Project Plan"
author: "the shared"
component: All
tags: [change-me]
created: 2026-02-19
updated: 2026-02-19
state: Active
supersedes: null
superseded-by: null
version: 1.1
---

# dendryform — Project Plan

> **Project:** `dendryform` — Declarative software architecture diagrams
>
> **Status:** Approved for development
>
> **Authors:** Duncan + Claude
>
> **Date:** February 2026

---

## Overview

dendryform takes a declarative description of a software system — nodes, edges,
containment, tiers — and renders it as a beautiful, interactive HTML architecture
diagram. It also exports to SVG, PNG, ASCII, Structurizr DSL, and Mermaid.

Named for the 23 dendriform models of tree architecture (Hallé & Oldeman, 1970),
because every system has a branching pattern worth revealing.

### Design Principles

1. **Human-readable first** — slug IDs, YAML authoring, clear error messages
2. **Author-controlled layout** — explicit tiers, not auto-layout black boxes
3. **The Taproot aesthetic** — dark-themed, tiered, color-coded, tech-badged, gorgeous
4. **Responsive HTML, pixel-perfect SVG** — shared layout plan, format-native rendering
5. **Standards-friendly** — lossy export to C4/Structurizr and Mermaid for interop
6. **Rust quality** — private fields, validated constructors, newtypes, doc comments,
   comprehensive tests, no anti-patterns

### Key Decisions (from research & evaluation)

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Node IDs | Human-readable slugs | Primary authoring format; UUID generated on Structurizr export |
| Font strategy | JetBrains Mono everywhere | Monospace sidesteps text measurement in SVG; matches aesthetic |
| Template engine | TBD (Phase 1 milestone) | Evaluate Askama vs Tera during implementation |
| Auto-layout | Not in scope | Author-controlled tiers are the core philosophy |
| Layout architecture | Shared LayoutPlan → format-native renderers | HTML stays responsive; SVG commits to absolute coords |
| Dependencies | serde, serde_json, serde_yml, thiserror, uuid, clap, resvg | No lazy_static, no serde_yaml 0.9, no heavy runtimes |
| structurizr-rs | REFERENCE only | Study for design ideas; no code extraction |
| Edition | 2024 | Latest stable Rust edition |

---

## Workspace Structure

```
dendryform/
├── Cargo.toml                  # Workspace root
├── crates/
│   ├── dendryform-core/        # Schema types, validation, theme, layout plan
│   ├── dendryform-parse/       # YAML + JSON → Diagram IR
│   ├── dendryform-layout/      # Diagram IR → LayoutPlan (shared positioning logic)
│   ├── dendryform-html/        # LayoutPlan → responsive HTML
│   ├── dendryform-svg/         # LayoutPlan → static SVG
│   ├── dendryform-png/         # SVG → PNG (resvg wrapper)
│   ├── dendryform-ascii/       # LayoutPlan → ASCII art
│   ├── dendryform-export/      # Lossy exporters (Structurizr DSL, JSON, Mermaid)
│   └── dendryform-cli/         # CLI binary
├── themes/                     # Built-in theme definitions
│   └── dark.yml                # The Taproot dark theme (default)
├── examples/                   # Example diagram YAML files
│   └── taproot.yml             # The reference diagram
└── tests/                      # Integration / snapshot tests
    ├── snapshots/              # Expected outputs for snapshot testing
    └── fixtures/               # Test input files
```

---

## Phase 1: Foundation (Core + Parse + HTML)

**Goal:** `dendryform render taproot.yml` produces the exact Taproot diagram.

This is the MVP. Everything else builds on this. Each milestone is a
self-contained Claude Code task that can be built and tested independently.

### Milestone 1.1 — Workspace Scaffold

**Scope:** Create the Cargo workspace, crate stubs, CI-ready structure.

- [ ] Workspace `Cargo.toml` with all crate members
- [ ] Each crate: `Cargo.toml` + `src/lib.rs` (or `main.rs`) with module stubs
- [ ] Shared workspace dependencies in `[workspace.dependencies]`
- [ ] `rustfmt.toml` and `clippy.toml` with our standards
- [ ] `.github/workflows/ci.yml` — cargo check, clippy, test, fmt
- [ ] `README.md` updated from placeholder
- [ ] `CLAUDE.md` with project context for Claude Code

**Acceptance:** `cargo check --workspace` succeeds. `cargo clippy --workspace`
has zero warnings. CI pipeline green.

**Estimated size:** ~200 lines of config/boilerplate. Small task.

---

### Milestone 1.2 — Core Schema Types (`dendryform-core`)

**Scope:** The data model — all Rust types for describing a diagram.

- [ ] `NodeId` newtype (validated slug: lowercase alphanumeric + dots + hyphens)
      — impl `Display`, `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`
- [ ] `Node` with private fields, consuming builder pattern (`self`, not `&mut self`),
      validated constructor
- [ ] `NodeKind` enum (Person, System, Container, Component, Infrastructure, Group)
      — `#[non_exhaustive]`, derive `Copy` (small enum), impl `Display`
- [ ] `Edge` with `NodeId` references, validated constructor
- [ ] `EdgeKind` enum (Uses, Reads, Writes, Deploys, Contains)
      — `#[non_exhaustive]`, derive `Copy`, impl `Display`
- [ ] `Tier` with label, nodes, layout hint
- [ ] `TierLayout` enum (Grid { columns }, Auto, Single)
      — `#[non_exhaustive]`
- [ ] `Connector` type for inter-tier visual connectors
- [ ] `Container` type for nested sub-diagrams within a tier
- [ ] `Diagram` — validated wrapper around a raw deserialized structure.
      Serde deserializes into `RawDiagram` (unchecked), then
      `Diagram::try_from(raw)` or `#[serde(try_from = "RawDiagram")]` validates
      invariants (no duplicate IDs, no dangling edge refs, no empty tiers).
      The `Diagram` type itself cannot represent an invalid state.
- [ ] `Tech` newtype for technology badges (not raw String) — impl `Display`
- [ ] `Color` enum/newtype for the named palette (Blue, Green, Amber, Purple, Red,
      Teal + Custom) — `#[non_exhaustive]`, impl `Display`
- [ ] `Metadata` newtype wrapping `HashMap<String, String>` (extensibility escape hatch)
      — explicit accessor methods (`get`, `insert`, `iter`, `is_empty`);
      do NOT impl `Deref<Target = HashMap>` (AP-15)
- [ ] All types derive `Debug, Clone, PartialEq` at minimum; `Eq, Hash` on ID types
      and small enums; `Copy` on small enums; `Serialize, Deserialize` on all
- [ ] `serde(rename_all = "snake_case")` for YAML-friendly field names
- [ ] `serde(default)` and `skip_serializing_if` where appropriate
- [ ] Doc comments on every public type, field, and method
- [ ] Builder methods consume `self` (AP-11: consuming builder prevents reuse bugs)
- [ ] Unit tests for builders, validation, serialization round-trips
- [ ] Test: invalid NodeId rejected (spaces, uppercase, empty)
- [ ] Test: Diagram with duplicate NodeIds rejected
- [ ] Test: Diagram with dangling edge reference rejected

**Acceptance:** All types compile, serialize/deserialize correctly.
`cargo test -p dendryform-core` passes with >95% coverage of public API.

**Estimated size:** ~600-800 lines of Rust + ~300 lines of tests. Medium task.

---

### Milestone 1.3 — Theme System (`dendryform-core`)

**Scope:** Theme types and the built-in Taproot dark theme.

- [ ] `Theme` struct with private fields, builder
- [ ] `ColorSet` struct (accent, hover_border, icon color)
- [ ] `ThemePalette` — named color sets keyed by `Color` enum
- [ ] Font configuration (display font, body font)
- [ ] Spacing/radius configuration
- [ ] Animation toggle
- [ ] Built-in `dark` theme via `Theme::dark()` constructor function (not `const` —
      types contain `String`/`Vec`; not `lazy_static` — use `std::sync::LazyLock`
      only if a global static is truly needed, otherwise just call the constructor)
- [ ] Theme loading from YAML file
- [ ] Theme merge semantics (user overrides on top of built-in)
- [ ] `themes/dark.yml` — the reference theme in YAML form
- [ ] Doc comments, unit tests, serialization round-trip tests
- [ ] Test: custom theme overrides specific palette entries

**Acceptance:** `Theme::dark()` returns the exact Taproot color values.
Theme YAML round-trips correctly. Merge produces expected overrides.

**Estimated size:** ~400-500 lines + ~150 lines tests. Small-medium task.

---

### Milestone 1.4 — YAML Parser (`dendryform-parse`)

**Scope:** Parse YAML input into validated `Diagram` IR.

- [ ] `parse_yaml(input: &str) -> Result<Diagram, ParseError>`
- [ ] `parse_yaml_file(path: impl AsRef<Path>) -> Result<Diagram, ParseError>`
      (API-03: accept `impl AsRef<Path>`, not `&Path`)
- [ ] `ParseError` type with line/column info where possible, using `thiserror`
      with `#[from]` for serde_yml and IO error conversion (EH-01, EH-07)
- [ ] Handle theme resolution: `theme: dark` → built-in, `theme: ./path` → load file
- [ ] Handle the full YAML structure from the design sketch:
  - Tiers with nodes
  - Tiers with containers (nested sub-tiers)
  - Tiers with connectors
  - Grid layout specs
  - Edge definitions
- [ ] Validation pass after deserialization:
  - All edge `from`/`to` reference existing NodeIds
  - No duplicate NodeIds across entire diagram
  - No empty tiers
  - Container nesting depth limit (e.g., max 3 levels)
- [ ] Write `examples/taproot.yml` — the complete Taproot diagram in YAML
- [ ] Doc comments, tests
- [ ] Test: parse taproot.yml successfully
- [ ] Test: parse error on missing node reference in edge
- [ ] Test: parse error on duplicate node ID
- [ ] Test: parse error on malformed YAML

**Acceptance:** `examples/taproot.yml` parses to a valid `Diagram`.
Error messages include context about what went wrong and where.

**Estimated size:** ~500-600 lines + ~200 lines tests + ~150 lines YAML. Medium task.

**Note:** JSON parsing (`parse_json`) can be added trivially since serde handles
both — include it here if easy, or defer to a later milestone.

---

### Milestone 1.5 — Layout Engine (`dendryform-layout`)

**Scope:** Transform Diagram IR into a LayoutPlan with logical positioning.

- [ ] `LayoutPlan` struct — the positioned output
- [ ] `LayoutError` — per-crate error type using `thiserror` (EH-07),
      with `From` impls for `?` propagation from core errors (EH-01)
- [ ] `TierGeometry` — grid spec, gap ratios, sizing weights per tier
      (named `TierGeometry`, NOT `TierLayout`, to avoid collision with the
      `TierLayout` enum in dendryform-core which represents the user's intent)
- [ ] `NodeGeometry` — grid column/row, column span, relative weight, nested plan
- [ ] `ConnectorGeometry` — position, label, style (line vs dots)
- [ ] `ContainerGeometry` — bounding area, nested layout plan
- [ ] `LegendGeometry` — items with color swatches and labels
- [ ] `compute_layout(diagram: &Diagram) -> Result<LayoutPlan, LayoutError>`
- [ ] Logic: walk tiers top-to-bottom, assign grid positions based on TierLayout
- [ ] Logic: compute connector positions between tiers
- [ ] Logic: recurse into containers for nested layout
- [ ] Logic: compute legend entries from color usage in diagram
- [ ] `ViewportHint` — preferred aspect ratio, reference width (1100px default)
- [ ] For SVG: `resolve_absolute(plan: &LayoutPlan, width: f32) -> Result<AbsoluteLayout, LayoutError>`
  - Monospace font metrics (JetBrains Mono character width at given size)
  - Concrete x, y, width, height for every element
  - Text wrapping decisions based on available box width
- [ ] Doc comments, tests
- [ ] Test: single-tier grid layout produces correct column assignments
- [ ] Test: container nesting produces recursive layout
- [ ] Test: absolute resolution at 1100px matches expected coordinates

**Acceptance:** Taproot diagram produces a LayoutPlan that accounts for all
tiers, containers, connectors, and the legend.

**Estimated size:** ~700-900 lines + ~300 lines tests. Medium-large task.

---

### Milestone 1.6 — HTML Renderer (`dendryform-html`)

**Scope:** Render LayoutPlan to self-contained, responsive HTML.

- [ ] `render_html(plan: &LayoutPlan, theme: &Theme) -> Result<String, RenderError>`
      (AP-28: rendering can fail — template errors, malformed data)
- [ ] `RenderError` — per-crate error type using `thiserror` (EH-07)
- [ ] Decide: Askama (compile-time) vs Tera (runtime) — evaluate both, pick one
  - If Askama: templates in `templates/` dir, compiled into binary
  - If Tera: templates loaded at runtime, supports custom user templates
- [ ] Template structure:
  - Base HTML shell (doctype, head, meta, style block, body)
  - CSS generation from Theme (CSS custom properties / variables)
  - Tier template (label, grid container)
  - Node template (card with color class, icon, title, description, tech badges)
  - Connector template (line + protocol label)
  - Container template (bordered area with floating label + nested tiers)
  - Internal connector template (dot pattern)
  - Flow labels template (arrow + label between tiers)
  - Legend template
- [ ] Responsive CSS: media queries for mobile collapse
- [ ] Hover states and animations (controlled by theme.animate)
- [ ] Font strategy: embed JetBrains Mono + DM Sans as base64 WOFF2 in the HTML
      (a Google Fonts import would contradict the self-contained requirement;
      alternatively, provide a `--fonts external` flag for network-dependent mode)
- [ ] Output: single self-contained HTML file (no external dependencies)
- [ ] Doc comments, tests
- [ ] **Snapshot test: render taproot.yml → compare against reference HTML**
- [ ] Test: HTML is valid (well-formed tags)
- [ ] Test: theme colors appear as CSS variables

**Acceptance:** `dendryform render examples/taproot.yml` produces HTML that is
visually identical to the original hand-crafted `taproot-architecture.html`.
Side-by-side comparison in a browser should show no meaningful differences.

**Estimated size:** ~800-1000 lines + templates + ~200 lines tests. Large task.

---

### Milestone 1.7 — CLI (`dendryform-cli`)

**Scope:** The command-line interface for Phase 1.

- [ ] `dendryform render <input> [-o output] [-f format]` — format defaults to html
- [ ] `dendryform validate <input>` — parse and validate without rendering
- [ ] `dendryform init [--template basic]` — generate starter YAML to stdout
- [ ] `dendryform themes` — list available built-in themes
- [ ] `--theme <name-or-path>` flag on render
- [ ] `--width <pixels>` flag for SVG/PNG viewport (default 1100)
- [ ] Clap v4 with derive API
- [ ] Colored terminal output for errors (use `anstream` or similar)
- [ ] Exit codes: 0 success, 1 validation error, 2 IO/parse error
- [ ] Doc comments, basic CLI tests
- [ ] Man page generation or `--help` that's comprehensive
- [ ] Test: render command produces output file
- [ ] Test: validate command exits 0 on valid input, 1 on invalid
- [ ] Test: init produces parseable YAML

**Acceptance:** Full CLI workflow works end-to-end:
`dendryform render examples/taproot.yml -o taproot.html` produces the diagram.

**Estimated size:** ~300-400 lines + ~100 lines tests. Small-medium task.

---

### Milestone 1.8 — Phase 1 Integration & Polish

**Scope:** End-to-end testing, documentation, release prep.

- [ ] Integration test: YAML → parse → layout → HTML → snapshot comparison
- [ ] Write 2-3 additional example diagrams (simple, medium complexity)
- [ ] `README.md` with usage examples, screenshots, YAML reference
- [ ] `CHANGELOG.md` initialized
- [ ] Workspace-level `cargo doc` builds cleanly with no warnings
- [ ] All `cargo clippy` warnings resolved
- [ ] `cargo test --workspace` all green
- [ ] Crate-level READMEs for each sub-crate
- [ ] License headers in all source files
- [ ] Publish `dendryform-core` and `dendryform` to crates.io (if ready)

**Acceptance:** A developer can `cargo install dendryform-cli`, write a YAML file
following the README, and produce a beautiful HTML diagram. Phase 1 complete.

**Estimated size:** ~400-500 lines of docs/tests/polish. Medium task.

---

## Phase 2: Visual Outputs (SVG + PNG + ASCII)

**Goal:** Three additional output formats, all driven by the shared LayoutPlan.

### Milestone 2.1 — SVG Renderer (`dendryform-svg`)

**Scope:** Static SVG output that matches HTML appearance at a fixed viewport.

- [ ] `render_svg(plan: &LayoutPlan, theme: &Theme, width: f32) -> Result<String, SvgError>`
      (AP-28: rendering can fail)
- [ ] `SvgError` — per-crate error type using `thiserror` (EH-07)
- [ ] Uses `AbsoluteLayout` from dendryform-layout
- [ ] SVG elements: `<rect>`, `<text>`, `<line>`, `<path>`, `<g>` grouping
- [ ] Monospace text layout using JetBrains Mono character metrics
- [ ] Embedded font via `@font-face` with base64 WOFF2, or `<style>` with web import
- [ ] Color-coded node cards with rounded corners and accent top-bar
- [ ] Tech badges as styled `<rect>` + `<text>` groups
- [ ] Connector lines with arrowheads (SVG `<marker>`)
- [ ] Container borders (solid for server, dashed for knowledge engine)
- [ ] Legend at bottom
- [ ] Output: single self-contained SVG file
- [ ] Snapshot test: SVG output at 1100px matches expected
- [ ] Visual comparison: SVG opened in browser matches HTML at same width

**Acceptance:** `dendryform render taproot.yml -f svg` produces an SVG that
looks like the HTML screenshot when viewed at 1100px width.

**Estimated size:** ~800-1000 lines + ~200 lines tests. Large task.

---

### Milestone 2.2 — PNG Renderer (`dendryform-png`)

**Scope:** Rasterize SVG to PNG via resvg.

- [ ] `render_png(svg: &str, scale: f32) -> Result<Vec<u8>, PngError>`
      (AP-28: resvg can fail on bad SVG, missing fonts, memory)
- [ ] `PngError` — per-crate error type using `thiserror` (EH-07)
- [ ] Integration with resvg + tiny-skia
- [ ] Configurable DPI / scale factor (1x, 2x for retina)
- [ ] Font loading for resvg (bundle JetBrains Mono or load from system)
- [ ] CLI flag: `dendryform render taproot.yml -f png --scale 2`
- [ ] Test: PNG output is non-empty and has expected dimensions

**Acceptance:** `dendryform render taproot.yml -f png` produces a crisp PNG.

**Estimated size:** ~200-300 lines + ~50 lines tests. Small task.

---

### Milestone 2.3 — ASCII Renderer (`dendryform-ascii`)

**Scope:** Lossy text rendering for terminals and code comments.

- [ ] `render_ascii(plan: &LayoutPlan, width: usize) -> Result<String, AsciiError>`
      (AP-28: rendering can fail)
- [ ] `AsciiError` — per-crate error type using `thiserror` (EH-07)
- [ ] Box-drawing characters for containers and nodes (`┌─┐│└─┘`)
- [ ] Tier labels as uppercase headers
- [ ] Node cards: name + abbreviated description
- [ ] Tech badges: omitted or comma-separated on one line
- [ ] Connectors: `│` with optional label
- [ ] Grid layout quantized to character columns
- [ ] Configurable width (default 120 columns, option for 80)
- [ ] Lossy decisions: collapse deeply nested containers, truncate long text
- [ ] Test: ASCII output for taproot diagram matches expected snapshot
- [ ] Test: 80-column mode truncates gracefully

**Acceptance:** `dendryform render taproot.yml -f ascii` produces readable
terminal output that captures the system structure.

**Estimated size:** ~500-700 lines + ~200 lines tests. Medium task.

---

### Milestone 2.4 — Phase 2 Integration

**Scope:** Wire new formats into CLI, update docs, snapshot tests.

- [ ] CLI: `-f svg`, `-f png`, `-f ascii` flags all functional
- [ ] `--width` flag applies to SVG and PNG
- [ ] `--scale` flag applies to PNG
- [ ] Snapshot tests for all three new formats against taproot.yml
- [ ] Update README with output format examples
- [ ] Example: render all formats from one YAML file

**Acceptance:** All four output formats work end-to-end from CLI.

**Estimated size:** ~200 lines of wiring + tests. Small task.

---

## Phase 3: Interop Exports

**Goal:** Lossy export to standards-friendly formats for ecosystem interop.

### Milestone 3.1 — Exporter Trait & Shared Filtering

**Scope:** Define the `Exporter` trait and shared logic for all interop formats.

- [ ] `ExportError` — per-crate error type using `thiserror` (EH-07)
- [ ] `LossyWarning` — structured type describing what was lost in translation
      (e.g., tier structure flattened, custom colors mapped to nearest C4 color)
- [ ] `ExportResult<T>` — bundles output + warnings:
  ```rust
  pub struct ExportResult<T> {
      output: T,           // private field
      warnings: Vec<LossyWarning>,  // private field
  }
  // with accessor methods: output(), warnings(), into_output()
  ```
- [ ] `Exporter` trait:
  ```rust
  pub trait Exporter {
      type Output;
      fn export(&self, diagram: &Diagram) -> Result<ExportResult<Self::Output>, ExportError>;
  }
  ```
  Note: single `ExportError` type for all exporters (not an associated type)
  since export errors share common structure. If exporters need format-specific
  error variants, use an enum with `#[non_exhaustive]`.
- [ ] Shared filtering logic: determine visible elements for a given view/scope
- [ ] Shared ID mapping: slug → UUID for Structurizr compat
- [ ] `NodeKind` → C4 element type mapping table
- [ ] `EdgeKind` → relationship string mapping

**Acceptance:** Trait compiles, mapping tables are complete and tested.

**Estimated size:** ~300-400 lines + ~150 lines tests. Small-medium task.

---

### Milestone 3.2 — Structurizr DSL Export

**Scope:** Export to Structurizr DSL text format.

- [ ] `StructurizrDslExporter` implementing `Exporter`
- [ ] Map Diagram → `workspace { model { ... } views { ... } styles { ... } }`
- [ ] People, software systems, containers, components with descriptions + tech
- [ ] Relationships with labels
- [ ] `autoLayout` hint in views (since we lose tier structure)
- [ ] Basic styles block mapping our colors to Structurizr element styles
- [ ] Test: taproot.yml exports to valid Structurizr DSL
- [ ] Test: exported DSL can be parsed by structurizr-rs (if we keep it around)

**Acceptance:** `dendryform export taproot.yml -f structurizr` produces valid DSL.

**Estimated size:** ~400-500 lines + ~150 lines tests. Medium task.

---

### Milestone 3.3 — Structurizr JSON Export

**Scope:** Export to Structurizr workspace JSON format.

- [ ] `StructurizrJsonExporter` implementing `Exporter`
- [ ] Full workspace JSON structure with model, views, configuration
- [ ] UUID generation for element IDs (v5 from slug for determinism)
- [ ] Test: output validates against Structurizr JSON schema

**Acceptance:** `dendryform export taproot.yml -f structurizr-json` produces
valid workspace JSON that could be imported into Structurizr.

**Estimated size:** ~300-400 lines + ~100 lines tests. Small-medium task.

---

### Milestone 3.4 — Mermaid Export

**Scope:** Export to Mermaid diagram syntax.

- [ ] `MermaidExporter` implementing `Exporter`
- [ ] Map to `graph TD` or `C4Context` Mermaid syntax
- [ ] Nodes as labeled boxes with descriptions
- [ ] Edges with labels
- [ ] Subgraphs for containers
- [ ] Test: output renders in GitHub Markdown preview
- [ ] Test: output renders in Mermaid Live Editor

**Acceptance:** `dendryform export taproot.yml -f mermaid` produces Mermaid
syntax that renders a recognizable (if visually simpler) diagram.

**Estimated size:** ~300-400 lines + ~100 lines tests. Small-medium task.

---

### Milestone 3.5 — Phase 3 Integration

**Scope:** Wire exporters into CLI, update docs.

- [ ] CLI: `dendryform export <input> -f <format> [-o output]`
- [ ] Formats: `structurizr`, `structurizr-json`, `mermaid`
- [ ] Print lossy conversion warnings to stderr
- [ ] Update README with export examples
- [ ] Integration tests for all export formats

**Acceptance:** All export formats work end-to-end from CLI.

**Estimated size:** ~200 lines of wiring + tests. Small task.

---

## Phase 4: Ecosystem (Future)

Not scoped in detail yet. Potential milestones:

- **4.1** — Custom theme support (load user YAML themes, light theme built-in)
- **4.2** — Workspace / multi-view support (one YAML, multiple diagram views)
- **4.3** — Taproot integration (MCP tool: `dendryform_render`)
- **4.4** — `dendryform suggest` — propose tier layout from flat node+edge list
- **4.5** — Watch mode (`dendryform watch diagram.yml` → live-reload HTML)
- **4.6** — Rust builder API (`Diagram::builder().tier(...)`)

---

## Summary

| Phase | Milestones | Estimated Total Lines | Focus |
|-------|-----------|----------------------|-------|
| **1** | 1.1–1.8 | ~4,000-5,000 | Foundation: schema, parser, layout, HTML, CLI |
| **2** | 2.1–2.4 | ~2,000-2,500 | Visual outputs: SVG, PNG, ASCII |
| **3** | 3.1–3.5 | ~1,500-2,000 | Interop: Structurizr, Mermaid |
| **4** | 4.1–4.6 | TBD | Ecosystem: themes, integration, tooling |

Phase 1 is the critical path. When it's done, `dendryform render taproot.yml`
produces the exact diagram that blew everyone's minds. Everything after that
is expanding reach and interoperability.

---

*Let's build the 24th architectural model.*
