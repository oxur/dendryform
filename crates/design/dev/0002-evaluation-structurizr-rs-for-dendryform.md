# Evaluation: structurizr-rs for dendryform

**Date:** 2026-02-19
**Evaluator:** Claude Code (per task 0001)
**Repository:** https://github.com/Helms-AI/structurizr-rs
**Commit:** latest on main at time of evaluation

---

## Step 1: Clone and Survey

### Overview

| Metric | Value |
|--------|-------|
| Total lines of Rust | ~59,000 |
| Total .rs files | 107 |
| Internal crates | 10 |
| Total tests | ~280 |
| Integration test files | 3 |
| Files >500 lines | 22 |
| Largest file | `handlers.rs` (11,889 lines) |
| Edition | 2021 |
| License | MIT |

### Crate Structure and Dependency Graph

```
Foundation Layer:
  structurizr-core (no internal deps)
  structurizr-config (no internal deps)

Parsing Layer:
  structurizr-dsl → core, scripting (optional)

Rendering/Export Layer:
  structurizr-render → core
  structurizr-export → core

Scripting Layer:
  structurizr-scripting → core

Analysis Layer:
  structurizr-analysis → core, dsl
  structurizr-github → core, dsl, analysis

Integration Layer:
  structurizr-web → core, dsl, render, export, config, github, analysis
  structurizr-mcp → core, dsl, render, export, web

Root Binary (src/main.rs):
  → core, dsl, render, export, web, scripting, mcp
```

The dependency graph is **clean and layered** — no circular dependencies, clear separation of concerns. `structurizr-core` is the foundation, with rendering and export as independent peers.

### Compilation

- **Compiles cleanly** — zero errors, zero warnings on `cargo check`.
- Build time ~30s (many transitive deps from wasmtime, tantivy, octocrab).

### Clippy Results

- **1 error**: `clippy::never_loop` in `navigation.rs:190` — a `for` loop that only ever executes once (should be `if let Some(...)`). This is a real bug.
- **9 warnings**:
  - 6x `clone_on_copy` — calling `.clone()` on `ElementId` which is `Copy`
  - 2x `extend_with_drain` — should use `.append()` instead
  - 1x `too_many_arguments` — `add_relationship_with_metadata()` takes 8 args
  - 1x `new_without_default` — `CrdtSession::new()` without `Default` impl

**Assessment:** Minor issues. The `never_loop` is a real bug but non-critical. The `clone_on_copy` issues are cosmetic. Overall, clippy hygiene is decent but not rigorous.

### Test Coverage

| Crate | Tests | Notes |
|-------|-------|-------|
| structurizr-render | 88 | Best covered — layout algorithms |
| structurizr-web | 55 | Handler tests |
| structurizr-dsl | 38 | Unit + 2 integration test files |
| structurizr-scripting | 22 | Lua/Groovy tests |
| structurizr-core | 20 | Basic model tests |
| structurizr-analysis | 20 | + 1 integration test file |
| structurizr-export | 13 | Per-format tests |
| structurizr-github | 8 | Minimal |
| structurizr-config | 4 | Minimal |
| structurizr-mcp | 0 | **No tests** |

**Assessment:** ~280 tests total for ~59K LOC is **sparse** — roughly 1 test per 210 lines. The render crate is well-tested; core and export are undertested. No property-based testing, no snapshot testing. Most tests are basic "it compiles and produces output" rather than deep behavioral verification.

---

## Step 2: Core Type Analysis (`structurizr-core`)

### Data Model

**C4 Hierarchy** — implemented via nested `Vec` ownership:
- `Model` owns `Vec<Person>`, `Vec<SoftwareSystem>`, `Vec<DeploymentNode>`, `Vec<CustomElement>`
- `SoftwareSystem` owns `Vec<Container>`
- `Container` owns `Vec<Component>`
- `DeploymentNode` owns `Vec<DeploymentNode>` (recursive), `Vec<ContainerInstance>`, `Vec<SoftwareSystemInstance>`

**Relationships** — stored as a flat `Vec<Relationship>` in `Model`, referencing source/destination by `ElementId`. This is a standard adjacency-list-as-flat-vec pattern.

**Containment** — pure composition via nested Vecs. No parent pointers, no trait objects. Simple and correct, though tree traversal requires recursive `find_element()`.

**Identity** — `ElementId(Uuid)` newtype wrapping UUID. Supports:
- `new()` — random UUID v4
- `from_name()` — deterministic UUID v5 (reproducible from name strings)
- Derives: `Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize`

This is **well-designed**. Copy semantics on a UUID newtype is exactly right.

### Type Design Quality

**Strengths:**
- `ElementId` newtype prevents stringly-typed ID confusion
- Small enums (`Location`, `InteractionStyle`, `Shape`, `Routing`, etc.) are `Copy` with proper serde
- `ElementProperties` uses `#[serde(flatten)]` for clean composition
- Builder pattern on all major types with `impl Into<String>` for ergonomic construction
- Strategic `Option<T>` usage with `skip_serializing_if`

**Weaknesses:**
- All struct fields are `pub` — no encapsulation of invariants (AP-06, AP-71). While intentional for a data-model crate, it means any consumer can construct invalid states.
- `properties: HashMap<String, String>` and `perspectives: HashMap<String, String>` are stringly-typed maps (AP-30, AP-53). Could benefit from newtype keys.
- `tags: Vec<String>` is stringly-typed — a `Tag` newtype would be better.
- `Relationship.interaction_style` defaults to `Synchronous` which may mask unset values.

### Serde Usage

**Excellent.** Comprehensive and consistent use of:
- `#[serde(flatten)]` for property composition
- `#[serde(rename_all = "lowercase")]` and `"PascalCase"` for enums
- `#[serde(default, skip_serializing_if = "Option::is_none")]` everywhere
- `#[serde(skip)]` for runtime-only fields (e.g., `dirty`, `last_change_timestamp`)
- `#[serde(rename = "workspaceConfiguration")]` for JSON field naming

The JSON output is clean, human-readable, and round-trips correctly.

---

## Step 3: Code Quality Audit

### Anti-Pattern Analysis

| Anti-Pattern | Occurrences | Severity | Location |
|-------------|-------------|----------|----------|
| **AP-06/AP-71: All pub fields** | Pervasive | Medium | All model types in `model.rs`, `view.rs`, `style.rs`, `workspace.rs` |
| **AP-30/AP-53: String for everything** | Moderate | Medium | `tags: Vec<String>`, `properties: HashMap<String, String>`, `perspectives: HashMap<String, String>` throughout |
| **AP-12/AP-57: clone_on_copy** | 6 instances | Minor | `view.rs:120,129,155,159,175,179` |
| **AP-09/AP-80: unwrap in lib** | Low | Minor | Theme cache lock `.unwrap()` in `theme.rs` — panics on poisoned mutex |
| **AP-17: Allocate in loops** | Not observed | — | — |
| **AP-08: to_string in hot loops** | Not observed | — | — |

**Notable absence of anti-patterns:**
- No unsafe code anywhere
- No `Box<dyn Error>` without Send+Sync
- No Deref polymorphism
- No Rc in async code
- No glob re-exports
- No deeply nested generics
- Clean error types with thiserror

### Error Handling

**Approach:** `thiserror` derive macro with domain-specific error enums per crate.

```rust
// structurizr-core/src/error.rs
#[derive(Error, Debug)]
pub enum Error {
    #[error("Element not found: {0}")]
    ElementNotFound(String),
    #[error("Invalid relationship: {0}")]
    InvalidRelationship(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    // ...
}
pub type Result<T> = std::result::Result<T, Error>;
```

**Assessment:** Good. Each crate has its own error type. Uses `#[from]` for automatic conversion. Error messages are descriptive. The DSL parser includes line/column location info in errors — excellent for user-facing error reporting.

**Weakness:** Some error variants use `String` payloads (e.g., `ElementNotFound(String)`) where an `ElementId` would be more precise.

### API Design

- **No trait hierarchy** — everything is concrete types. This is a deliberate choice for a data-model crate and is fine for C4-specific usage, but limits extensibility.
- Public API surface is clean: `lib.rs` uses explicit `pub use` re-exports.
- Builder pattern is consistent and ergonomic.
- No trait-based abstractions for exporters — each is an independent struct with static methods.

### Code Organisation

- Module structure is logical and well-separated.
- **Major concern:** `handlers.rs` at 11,889 lines is a monster file. This is clearly the web server's request handler and should be split.
- `parser.rs` at 4,038 lines is large but acceptable for a hand-written recursive descent parser.
- `svg.rs` at 3,272 lines is large — SVG generation is inherently verbose.

### Documentation

- **No doc comments** on public types or functions (spot-checked across multiple crates).
- **No doc tests.**
- README is fairly accurate relative to code capabilities.
- Internal code has occasional comments explaining algorithms (especially in layout/routing).

**Assessment:** Documentation is **poor** by our standards. Per our guide (13-documentation.md), all public items should have doc comments.

---

## Step 4: Renderer Analysis (`structurizr-render`)

### SVG Rendering Approach

**Programmatic string building** — not template-based. SVG elements are constructed via string concatenation and `format!()` calls in `svg.rs` (~3,272 lines). Each shape type has a dedicated render function.

### Layout Algorithm

**Sugiyama hierarchical layout** — a sophisticated 6-phase pipeline in `sugiyama/`:

1. **Cycle removal** (`cycle_removal.rs`, ~180 lines) — DFS-based back-edge reversal
2. **Layer/rank assignment** (`ranking.rs`, ~250 lines) — longest-path algorithm
3. **Dummy node insertion** (`dummy.rs`, ~200 lines) — splits long edges
4. **Crossing minimization** (`ordering.rs`, ~854 lines) — barycentric + weighted median + 2-opt local search with adaptive iteration counts
5. **Coordinate assignment** (`positioning.rs`, ~675 lines) — barycenter-based + force-directed refinement (15-20 iterations, 0.4 pull factor)
6. **Configuration** — direction-aware defaults per view type (C1: LeftRight, C2: TopBottom, C3: LeftRight)

**Assessment:** This is a **serious** layout implementation. The crossing minimization alone uses three distinct heuristics with adaptive iteration counts based on graph density. The force-directed refinement stage is a nice touch for visual polish.

### Text Measurement

**Rough estimation only** — width = char_count * font_size * 0.6, height = font_size * 1.2. No font metrics library. This is the biggest weakness in the renderer — text overflow/overlap is likely for non-ASCII text or variable-width fonts.

### Edge Routing

Three algorithms in `routing/`:
- **Direct** — straight lines with rectangle intersection
- **Curved** — cubic Bezier with JointJS-compatible control points, parallel edge spacing
- **Orthogonal** — right-angle paths with port distribution, optional A* pathfinding

### Collision Detection

**Quadtree-based** spatial indexing (`collision.rs`, ~850 lines) with AABB intersection, label placement optimization (8 preferred positions with scoring), and node separation via damping-based overlap resolution.

### Visual Quality

Based on the rendering architecture, output quality should be **good for auto-layout diagrams** — the Sugiyama layout is well-implemented, collision detection prevents overlaps, and edge routing handles parallel edges. The weak point is text measurement.

### What's Worth Learning

1. The Sugiyama layout pipeline structure and its 6-phase decomposition
2. Adaptive iteration counts based on graph density
3. Quadtree-based collision detection for label placement
4. The three routing algorithms and when each is appropriate
5. Force-directed refinement as a post-layout polish step

---

## Step 5: Export Analysis (`structurizr-export`)

### Exporter Structure

**Not trait-based** — each exporter is an independent struct with static methods. No shared `Exporter` trait. All exporters follow the same pattern:

1. Build `allowed_ids` from view elements
2. Build `candidate_ids` (including proxy candidates for containers/components)
3. Build `connected_ids` (relationships where both endpoints are candidates)
4. Emit format-specific output

This filtering algorithm is **duplicated** across all 7 exporters. A shared trait or helper function would eliminate ~500 lines of duplication.

### Per-Exporter Quality

| Exporter | Lines | Quality | Notes |
|----------|-------|---------|-------|
| JSON | ~25 | Simple/clean | Thin serde wrapper |
| Mermaid | ~800 | High | C4Context diagrams, proper sanitization |
| PlantUML | ~1000 | High | Uses C4_Context.puml stdlib |
| DOT | ~500 | Solid | Graphviz digraph with colors |
| D2 | ~700 | Good | Modern D2 syntax, tooltips |
| Ilograph | ~400 | Good | YAML via serde, nested resources |
| WebSequenceDiagrams | ~200 | Solid | Dynamic view → sequence diagrams |

### Separation from Core

**Clean** — exporters only read from `Workspace`/`Model`/`Views` via public fields. No mutation, no coupling beyond the core data types.

### Extractability for dendryform

The Mermaid and JSON exporters are the most relevant. The filtering algorithm (candidate + connected pattern) is worth extracting as a shared utility. The exporters themselves are straightforward string builders — easy to rewrite from scratch following the same pattern.

---

## Step 6: Dependency Audit

### Core Dependencies

| Dependency | Version | Maintained? | Would We Use? | Notes |
|-----------|---------|-------------|---------------|-------|
| **serde** | 1.0 | Yes, industry standard | Yes | Essential for any data model |
| **serde_json** | 1.0 | Yes | Yes | JSON serialization |
| **thiserror** | 1.0 | Yes, by dtolnay | Yes | Error derive macros |
| **uuid** | 1.0 | Yes | Likely | For element IDs; could also use ulid |
| **chrono** | 0.4 | Yes | Maybe | For timestamps; `time` crate is lighter |
| **ureq** | 2.10 | Yes | Maybe | Blocking HTTP for theme fetching; might prefer async |
| **lazy_static** | 1.5 | Yes but `std::sync::LazyLock` exists since Rust 1.80 | No | Use `std::sync::LazyLock` instead |
| **pest** | 2.7 | Yes | Maybe | PEG parser; we might prefer hand-written or winnow |
| **serde_yaml** | 0.9 | Deprecated | No | Use `serde_yml` instead |
| **mlua** | 0.10 | Yes | Unlikely | Lua runtime — heavy, niche |
| **wasmtime** | 27 | Yes | Unlikely | WASM runtime — very heavy |
| **axum** | 0.7 | Yes | Maybe | If we need a web server |
| **tantivy** | 0.22 | Yes | Unlikely | Full-text search — overkill for us |
| **octocrab** | 0.41 | Yes | Unlikely | GitHub API — not our use case |
| **tree-sitter** | 0.24 | Yes | Unlikely | Code analysis — not our use case |
| **rusqlite** | 0.32 | Yes | Unlikely | SQLite — not our use case |

### Red Flags

- **serde_yaml 0.9** is deprecated; should use `serde_yml`.
- **lazy_static** is obsolete since Rust 1.80's `std::sync::LazyLock`.
- **wasmtime 27** pulls in cranelift (a compiler backend) — massive transitive dependency tree for optional WASM plugin support.
- **pest 2.7** is listed in `structurizr-dsl/Cargo.toml` but the parser appears hand-written. May be a phantom dependency or used only for small pieces.

### Dependencies We'd Adopt

- `serde`, `serde_json`, `thiserror` — absolutely
- `uuid` — likely, with v4/v5 features
- `chrono` — maybe, depending on needs

### Dependencies We'd Reject

- `lazy_static` — use `std::sync::LazyLock`
- `serde_yaml` — use `serde_yml` if needed
- `wasmtime`, `mlua` — too heavy, not needed
- `tantivy`, `octocrab`, `rusqlite` — wrong domain

---

## Step 7: Synthesis — Recommendations for dendryform

### Reuse Assessment

| Component | Rating | Rationale |
|-----------|--------|-----------|
| **Core data model types** | **REFERENCE** | Well-designed C4 hierarchy with good serde. But all-pub fields and stringly-typed tags/properties don't meet our standards. Study the structure, rewrite with proper encapsulation. |
| **Relationship/edge model** | **REFERENCE** | Flat Vec with ElementId references is simple and correct. The `InteractionStyle` enum and `perspectives` field are good ideas. Worth studying, not extracting. |
| **DSL parser** | **REFERENCE** | 4K-line hand-written parser with good error reporting. The AST design and directive system are instructive. But we'll want our own DSL with different syntax. |
| **SVG renderer** | **REFERENCE** | String-based SVG generation is practical. The shape catalog (14 shapes) is a useful reference for what we need to support. Rewrite for cleanliness. |
| **Layout engine** | **REFERENCE** | The Sugiyama implementation is the most valuable part of the codebase. Study the 6-phase pipeline carefully. However, the implementation has large files and no doc comments — we'd rewrite with better structure. |
| **Mermaid exporter** | **REFERENCE** | Clean string-building approach. The candidate+connected filtering algorithm is worth understanding. Easy to rewrite. |
| **Structurizr JSON exporter** | **IGNORE** | Trivial serde wrapper — nothing to learn. |
| **Other exporters (PlantUML, D2, DOT)** | **REFERENCE** | Useful as format references when we implement our own exporters. |
| **Web server** | **IGNORE** | 11K-line handler file, tantivy search, CRDT collaboration — completely different architecture from what we'd build. |
| **CLI structure** | **REFERENCE** | Simple clap-based CLI with serve/validate/export/render/init commands. Useful as a feature checklist. |

### Key Takeaways

#### 1. Best Ideas to Steal (Conceptually)

1. **ElementId as UUID v5 newtype** — deterministic IDs from names allow reproducible serialization. This is clever and we should adopt it.
2. **Sugiyama layout pipeline** — the 6-phase decomposition (cycle removal → ranking → dummy nodes → crossing minimization → coordinate assignment → cleanup) is the standard approach and their implementation is solid.
3. **Adaptive layout parameters** — iteration counts based on graph density, view-type-specific layout defaults (C1: LeftRight, C2: TopBottom) are good UX decisions.
4. **Theme system with merge semantics** — workspace styles override theme styles, with tag-based matching. Simple and effective.
5. **Navigation index with breadcrumbs** — precomputed drill-down targets and breadcrumb chains for interactive navigation.
6. **LayoutState with undo/redo** — coalesced move operations (500ms window) with deque-based history.
7. **Candidate + connected filtering** — the algorithm for determining which elements to show in a view based on both element membership and relationship connectivity.

#### 2. Biggest Mistakes to Avoid

1. **All-pub fields without invariants** — Their model types allow construction of invalid states. We should use private fields with validated constructors.
2. **No trait abstractions for exporters** — Copy-pasting the same filtering algorithm across 7 exporters is maintenance debt. We should define an `Exporter` trait with shared filtering logic.
3. **Monster files** — `handlers.rs` (11,889 lines), `parser.rs` (4,038 lines), `svg.rs` (3,272 lines). We should keep files under 500 lines.
4. **No documentation** — Zero doc comments on public types. We must document everything.
5. **Stringly-typed tags and properties** — `Vec<String>` for tags and `HashMap<String, String>` for properties lose type safety. Use newtypes.
6. **Blocking HTTP in library code** — `theme.rs` uses `ureq` (blocking) for theme fetching. In async contexts this is problematic.
7. **Sparse test coverage** — ~280 tests for ~59K LOC. Most are basic smoke tests, not behavioral verification. We need >=95% coverage.

#### 3. Dependency Choices

**Adopt:**
- `serde` + `serde_json` — non-negotiable for serialization
- `thiserror` — the standard for library error types
- `uuid` — good choice for element IDs

**Explicitly Reject:**
- `lazy_static` — use `std::sync::LazyLock` (stable since Rust 1.80)
- `serde_yaml` 0.9 — deprecated, use `serde_yml`
- `pest` — if we write a parser, hand-write it or use `winnow`/`nom`
- `ureq` — prefer async HTTP if needed
- `wasmtime` — too heavy for plugin support (consider `wasmer` or skip WASM)

#### 4. Data Model Comparison

**What's Better About Theirs:**
- More complete C4 coverage (DeploymentNode with recursive children, InfrastructureNode, ContainerInstance, SoftwareSystemInstance, CustomElement)
- Perspectives system (multi-stakeholder viewpoints on relationships)
- Documentation/ADR support built into the model
- Terminology customization (rename "Container" to "Service", etc.)
- Branding (logo, custom font)

**What Should Be Better About Ours:**
- Private fields with validated constructors (type-safe invariants)
- Newtype tags instead of `Vec<String>`
- Stronger typing for properties (not just `HashMap<String, String>`)
- Trait-based extensibility for custom element types
- Doc comments on every public item
- >= 95% test coverage

### Overall Assessment

**Is this well-engineered?**

It is **competent but not exemplary**. The core data model is well-structured with good serde usage and a clean ElementId newtype. The Sugiyama layout engine is genuinely sophisticated. The 10-crate workspace decomposition is logical. However, the lack of documentation, sparse testing, all-pub fields, stringly-typed metadata, monster files, and duplicated export logic prevent it from being a high-quality reference implementation.

The codebase feels like it was **built rapidly by someone knowledgeable** — the architectural decisions are sound, but the polish (docs, tests, encapsulation, clippy hygiene) is missing. It has the hallmarks of AI-assisted development with competent human guidance: correct structure, good library choices, but insufficient attention to the details that make code maintainable long-term.

**Value to dendryform: 6/10**

The main value is:
- Confirming our data model approach (C4 hierarchy via composition)
- The Sugiyama layout pipeline as an algorithmic reference
- The shape catalog and export format specifications
- Theme/style merge semantics

**Estimated time saved:** 1-2 days of research. The layout algorithm reference alone saves significant time versus reading academic papers. But nothing here is worth extracting directly — everything should be rewritten to our standards.
