# dendryform

[![][build-badge]][build]
[![][crate-badge]][crate]
[![][tag-badge]][tag]
[![][docs-badge]][docs]
[![][license badge]][license]

*Declarative software architecture diagrams — beautiful, dark-themed, with a simple schema*

[![][logo]][logo-large]

Named for the 23 [dendriform models](https://en.wikipedia.org/wiki/Plant_architecture) of tree architecture (Halle & Oldeman, 1970), because every system has a branching pattern worth revealing.

## What It Does

`dendryform` takes a declarative YAML description of a software system — nodes, edges, containment, tiers — and renders it as a beautiful, dark-themed architecture diagram in HTML, SVG, or PNG.

[![Taproot architecture diagram][taproot-medium]][taproot-large]

click for full-size

## Quick Example

```yaml
diagram:
  title:
    text: "My API"
    accent: "My API"
  subtitle: "Two-tier service architecture"
  theme: dark

layers:
  - tier:
      id: clients
      label: "Clients"
      nodes:
        - id: web
          kind: person
          color: blue
          icon: "◇"
          title: "Web App"
          description: "React frontend"
          tech: ["TypeScript"]
  - connector:
      style: line
      label: "HTTPS"
  - tier:
      id: services
      label: "Services"
      nodes:
        - id: api
          kind: system
          color: green
          icon: "◈"
          title: "API Server"
          description: "REST endpoints"
          tech: ["Rust", "Axum"]
        - id: db
          kind: infrastructure
          color: amber
          icon: "◯"
          title: "PostgreSQL"
          description: "Primary store"
          tech: ["RDS"]

edges:
  - from: web
    to: api
    kind: uses
    label: "requests"
  - from: api
    to: db
    kind: reads

legend:
  - color: blue
    label: "Clients"
  - color: green
    label: "Services"
  - color: amber
    label: "Data"
```

## Usage

### 1 — Generate an `architecture.yaml` with Claude Code

The fastest way to diagram an existing codebase is to let an AI read the schema and your source, then write the YAML for you. Point Claude Code at the bundled schema reference:

```text
Read assets/schema/DIAGRAM-YAML-SCHEMA.md, then analyse the source code
in src/ and generate an architecture.yaml file that captures the system
architecture using the dendryform schema.
```

The schema document (`assets/schema/DIAGRAM-YAML-SCHEMA.md`) is written specifically for AI assistants — it covers every field, all valid values, and worked examples.

### 2 — Render to SVG or PNG

Once you have an `architecture.yaml`, render it with the CLI:

```bash
# PNG — format inferred from the output extension
dendryform render architecture.yaml -o diagram.png

# SVG
dendryform render architecture.yaml -o diagram.svg

# HTML (interactive)
dendryform render architecture.yaml -o diagram.html

# Explicit format flag
dendryform render architecture.yaml -f png -o diagram.png

# Retina PNG (2× scale)
dendryform render architecture.yaml -o diagram.png --scale 2.0
```

Run `dendryform --help` or `dendryform render --help` for all options including `--theme` and `--width`.

## Workspace Structure

```text
dendryform/
├── Cargo.toml                  # Workspace root
├── crates/
│   ├── dendryform-core/        # Schema types, validation, theme, layout plan
│   ├── dendryform-parse/       # YAML + JSON → Diagram IR
│   ├── dendryform-layout/      # Diagram IR → LayoutPlan
│   ├── dendryform-html/        # LayoutPlan → responsive HTML
│   ├── dendryform-svg/         # LayoutPlan → static SVG
│   ├── dendryform-png/         # SVG → PNG (resvg wrapper)
│   ├── dendryform-ascii/       # LayoutPlan → ASCII art (planned)
│   ├── dendryform-export/      # Lossy exporters: Structurizr DSL, JSON, Mermaid (planned)
│   └── dendryform-cli/         # CLI binary
├── assets/
│   └── schema/                 # DIAGRAM-YAML-SCHEMA.md — AI-readable schema reference
├── examples/                   # Example diagram YAML files and rendered outputs
└── tests/                      # Integration / snapshot tests
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.

[//]: ---Named-Links---

[logo]: assets/images/logo/v1-x250.png
[logo-large]: assets/images/logo/v1.png
[taproot-medium]: examples/taproot/diagram-medium.png
[taproot-large]: examples/taproot/diagram.png
[build]: https://github.com/oxur/dendryform/actions/workflows/ci.yml
[build-badge]: https://github.com/oxur/dendryform/actions/workflows/ci.yml/badge.svg
[crate]: https://crates.io/crates/dendryform
[crate-badge]: https://img.shields.io/crates/v/dendryform.svg
[docs]: https://docs.rs/dendryform/
[docs-badge]: https://img.shields.io/badge/rust-documentation-blue.svg
[tag-badge]: https://img.shields.io/github/tag/oxur/dendryform.svg
[tag]: https://github.com/oxur/dendryform/tags
[license]: LICENSE-APACHE
[license badge]: https://img.shields.io/badge/License-Apache%202.0%2FMIT-blue.svg
