# dendryform

> Declarative software architecture diagrams — beautiful, dark-themed, with a simple schema.

Named for the 23 [dendriform models](https://en.wikipedia.org/wiki/Plant_architecture) of tree architecture (Halle & Oldeman, 1970), because every system has a branching pattern worth revealing.

## Status

Early development — schema design and HTML renderer in progress.

## What This Will Be

`dendryform` takes a declarative description of a software system — nodes, edges, containment, tiers — and renders it as a beautiful, interactive HTML architecture diagram. It also exports to SVG, PNG, ASCII, Structurizr DSL, and Mermaid.

```yaml
title: "myproject - system architecture"
theme: dark

tiers:
  - label: Client Layer
    layout: single
    nodes:
      - id: web-app
        label: Web Application
        kind: system
        tech: [React, TypeScript]
```

```bash
dendryform render architecture.yaml -o architecture.html
```

## Workspace Structure

```
dendryform/
├── Cargo.toml                  # Workspace root
├── crates/
│   ├── dendryform-core/        # Schema types, validation, theme, layout plan
│   ├── dendryform-parse/       # YAML + JSON → Diagram IR
│   ├── dendryform-layout/      # Diagram IR → LayoutPlan
│   ├── dendryform-html/        # LayoutPlan → responsive HTML
│   ├── dendryform-svg/         # LayoutPlan → static SVG
│   ├── dendryform-png/         # SVG → PNG (resvg wrapper)
│   ├── dendryform-ascii/       # LayoutPlan → ASCII art
│   ├── dendryform-export/      # Lossy exporters (Structurizr DSL, JSON, Mermaid)
│   └── dendryform-cli/         # CLI binary
├── themes/                     # Built-in theme definitions
├── examples/                   # Example diagram YAML files
└── tests/                      # Integration / snapshot tests
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
