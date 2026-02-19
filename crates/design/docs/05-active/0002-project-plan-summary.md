---
number: 2
title: "Project Plan Summary"
author: "the shared"
component: All
tags: [change-me]
created: 2026-02-19
updated: 2026-02-19
state: Active
supersedes: null
superseded-by: null
version: 1.0
---

# Project Plan Summary

## Overview

dendryform is a Rust library and CLI that takes a declarative description of a software system (nodes, edges, containment, tiers) and renders it as a beautiful, dark-themed HTML architecture diagram. It also exports to SVG, PNG, ASCII, Structurizr DSL, and Mermaid.

The project is organized as a Cargo workspace with 9 crates (`dendryform-core`, `dendryform-parse`, `dendryform-layout`, `dendryform-html`, `dendryform-svg`, `dendryform-png`, `dendryform-ascii`, `dendryform-export`, `dendryform-cli`) across 4 phases totaling an estimated 7,500-9,500 lines of Rust.

Core principles: human-readable YAML authoring with slug IDs, author-controlled tiered layout (not auto-layout), the Taproot dark aesthetic, responsive HTML with pixel-perfect SVG, standards-friendly lossy export, and strict Rust quality standards (private fields, validated constructors, newtypes, doc comments, 95%+ test coverage).

---

## Phase 1: Foundation (Core + Parse + HTML)

The MVP phase. Goal: `dendryform render taproot.yml` produces the exact Taproot diagram. Establishes the workspace structure, core data model, theme system, YAML parsing, layout engine, HTML rendering, and CLI. Everything else builds on this.

| # | Name / Category | Description | Complexity | Crates |
|---|----------------|-------------|------------|--------|
| 1.1 | Workspace Scaffold | Cargo workspace, crate stubs, CI pipeline, rustfmt/clippy config, CLAUDE.md | Small | All (config) |
| 1.2 | Core Schema Types | NodeId newtype, Node/Edge/Tier/Connector/Container/Diagram types with builders, validation, serde, doc comments, tests | Medium | dendryform-core |
| 1.3 | Theme System | Theme/ColorSet/ThemePalette types, built-in Taproot dark theme, YAML theme loading, merge semantics | Small-Medium | dendryform-core |
| 1.4 | YAML Parser | Parse YAML into validated Diagram IR, ParseError with line/column info, validation pass (edge refs, duplicate IDs, empty tiers), taproot.yml example | Medium | dendryform-parse |
| 1.5 | Layout Engine | LayoutPlan computation from Diagram IR: grid positioning, connectors, container nesting, legend, ViewportHint, AbsoluteLayout resolution for SVG | Medium-Large | dendryform-layout |
| 1.6 | HTML Renderer | LayoutPlan + Theme to self-contained responsive HTML, template engine (Askama vs Tera), CSS from theme, hover/animation, snapshot tests | Large | dendryform-html |
| 1.7 | CLI | clap v4 derive API: render, validate, init, themes commands; colored errors, exit codes | Small-Medium | dendryform-cli |
| 1.8 | Integration & Polish | End-to-end snapshot tests, additional example diagrams, README, CHANGELOG, cargo doc, crate READMEs, crates.io prep | Medium | All |

**Estimated total:** ~4,000-5,000 lines

---

## Phase 2: Visual Outputs (SVG + PNG + ASCII)

Three additional output formats, all driven by the shared LayoutPlan from Phase 1. Extends the rendering pipeline without changing the core data model.

| # | Name / Category | Description | Complexity | Crates |
|---|----------------|-------------|------------|--------|
| 2.1 | SVG Renderer | Static SVG matching HTML appearance at fixed viewport; monospace text layout, embedded font, color-coded cards, tech badges, arrowhead markers, container borders, legend | Large | dendryform-svg |
| 2.2 | PNG Renderer | Rasterize SVG to PNG via resvg + tiny-skia; configurable DPI/scale, font loading | Small | dendryform-png |
| 2.3 | ASCII Renderer | Lossy text rendering with box-drawing characters; quantized grid, configurable width (80/120 col), graceful truncation | Medium | dendryform-ascii |
| 2.4 | Phase 2 Integration | Wire -f svg/png/ascii into CLI, --width and --scale flags, snapshot tests for all formats, README updates | Small | dendryform-cli |

**Estimated total:** ~2,000-2,500 lines

---

## Phase 3: Interop Exports

Lossy export to standards-friendly formats for ecosystem interoperability. Defines a shared `Exporter` trait with structured lossy-conversion warnings.

| # | Name / Category | Description | Complexity | Crates |
|---|----------------|-------------|------------|--------|
| 3.1 | Exporter Trait & Shared Logic | `Exporter` trait definition, shared filtering/visibility logic, slug-to-UUID mapping, NodeKind/EdgeKind to C4 mapping tables, lossy conversion warnings | Small-Medium | dendryform-export |
| 3.2 | Structurizr DSL Export | Diagram to `workspace { model { ... } views { ... } }` text format; people, systems, containers, relationships, autoLayout hints, basic styles | Medium | dendryform-export |
| 3.3 | Structurizr JSON Export | Full workspace JSON with UUID v5 IDs, model/views/configuration structure | Small-Medium | dendryform-export |
| 3.4 | Mermaid Export | Diagram to `graph TD` or `C4Context` syntax; labeled nodes, edges, subgraphs for containers | Small-Medium | dendryform-export |
| 3.5 | Phase 3 Integration | Wire `dendryform export` into CLI with -f structurizr/structurizr-json/mermaid, lossy warnings to stderr, integration tests, README updates | Small | dendryform-cli |

**Estimated total:** ~1,500-2,000 lines

---

## Phase 4: Ecosystem (Future)

Not yet scoped in detail. Extends dendryform with ecosystem features, tooling, and integrations.

| # | Name / Category | Description | Complexity | Crates |
|---|----------------|-------------|------------|--------|
| 4.1 | Custom Themes | User YAML theme loading, built-in light theme | Small | dendryform-core |
| 4.2 | Multi-View Workspaces | One YAML file, multiple diagram views | Medium | dendryform-core, dendryform-parse |
| 4.3 | Taproot MCP Integration | MCP tool: `dendryform_render` for AI assistant usage | Medium | New crate or dendryform-cli |
| 4.4 | Layout Suggestions | `dendryform suggest` — propose tier layout from flat node+edge list | Medium-Large | dendryform-layout |
| 4.5 | Watch Mode | `dendryform watch diagram.yml` with live-reload HTML in browser | Medium | dendryform-cli |
| 4.6 | Rust Builder API | Programmatic `Diagram::builder().tier(...)` for library consumers | Small-Medium | dendryform-core |

**Estimated total:** TBD
