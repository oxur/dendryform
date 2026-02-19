# Claude Code Task: Evaluate structurizr-rs for dendryform

## Context

We are building **dendryform** — a Rust library and CLI that renders software architecture diagrams from a declarative schema into beautiful, dark-themed HTML (and SVG, PNG, ASCII, Structurizr DSL, Mermaid). It is part of a broader Rust ecosystem (github.com/oxur: fabryk, confyg, twyg, etc.).

Before we start building, we want to evaluate **structurizr-rs** (`https://github.com/Helms-AI/structurizr-rs`), a Rust implementation of Structurizr Lite for C4 model diagrams. We want to understand:

1. What can we learn from it?
2. Is any of the code worth reusing, adapting, or extracting?
3. What should we avoid?

## Step 0: Read Your Rust Skills

Before doing anything else, read the Rust AI SKILL.md and its associated guides. Pay particular attention to:

- The **anti-patterns guide** — you'll be evaluating structurizr-rs against these patterns
- The **error handling guide** — how do they handle errors?
- The **type design guide** — how well-designed are their core types?
- Any other guides relevant to evaluating library architecture and code quality

These guides represent our team's Rust standards. structurizr-rs should be evaluated against them.

## Step 1: Clone and Survey

```bash
git clone https://github.com/Helms-AI/structurizr-rs.git /tmp/structurizr-rs
```

Get a high-level understanding of the repo:

- Total lines of code (use `tokei` or similar)
- Crate structure and dependency graph between internal crates
- External dependencies (`Cargo.toml` for each crate)
- Test coverage — how many tests, what kind (unit, integration, snapshot)?
- Does it compile cleanly? Any warnings?
- Run `cargo clippy` — how many warnings/lints?

## Step 2: Core Type Analysis (`structurizr-core`)

This is the most important crate for us. Evaluate deeply:

### Data Model

- How do they represent the C4 hierarchy (Person, SoftwareSystem, Container, Component)?
- How are relationships/edges modeled?
- How is containment (nesting) represented — tree structure, parent IDs, or something else?
- Is the model generic/extensible, or rigidly C4-specific?
- How do they handle identity — string IDs, generated IDs, typed IDs?

### Type Design Quality

- Are types well-designed per our Rust type design guide?
- Do they use newtypes, enums, and the type system effectively?
- Or is it stringly-typed with lots of `String` fields and runtime validation?
- How are optional fields handled — `Option<T>` appropriately, or defaults everywhere?
- Is there a clean separation between the data model and serialization concerns?

### Serde Usage

- How is serialization/deserialization implemented?
- Do they use `#[serde(rename_all)]`, `#[serde(default)]`, etc. appropriately?
- Is the serialized format clean and well-documented?

## Step 3: Code Quality Audit

Evaluate against our Rust anti-patterns guide. For each anti-pattern found, note:

- Which anti-pattern
- Where it occurs (file + approximate location)
- Severity (minor style issue vs. architectural problem)

Additionally evaluate:

### Error Handling

- Do they use `thiserror`, `anyhow`, custom error types, or raw strings?
- Is error handling consistent across crates?
- Are errors informative and actionable?

### API Design

- Are public APIs well-designed? Would you want to depend on them?
- Is there a clean trait hierarchy, or is everything concrete types?
- How is the crate boundary designed — are internal crates properly encapsulated?

### Code Organisation

- Is the module structure logical?
- Are files appropriately sized, or are there 2000+ line monsters?
- Is there dead code, commented-out code, or TODO sprawl?

### Testing

- What's the testing strategy?
- Are tests meaningful or just "it compiles" smoke tests?
- Any property-based testing, snapshot testing, or integration tests?

### Documentation

- Are public types and functions documented?
- Are there doc tests?
- Is the README accurate relative to what the code actually does?

## Step 4: Renderer Analysis (`structurizr-render`)

Since we're building renderers too:

- What's their SVG rendering approach? Template-based or programmatic?
- How do they handle layout — what algorithm, how configurable?
- How do they handle text measurement / bounding boxes in SVG?
- What's the visual quality of the output?
- Is there anything clever in their approach worth learning from?

## Step 5: Export Analysis (`structurizr-export`)

We want Structurizr DSL and Mermaid export. Evaluate:

- How are exporters structured — trait-based, or ad-hoc per format?
- Quality of the Mermaid exporter specifically
- Quality of the Structurizr JSON exporter
- Is the export logic cleanly separated from the core model?
- Could any of this be extracted and adapted for dendryform?

## Step 6: Dependency Audit

For each external dependency:

- What is it, what version?
- Is it well-maintained, widely used?
- Would we want to use the same crate for dendryform?
- Any red flags (unmaintained, too many transitive deps, etc.)?

## Step 7: Synthesis — Recommendations for dendryform

Based on all the above, provide a clear recommendation:

### Reuse Assessment

Rate each component as one of:

- **EXTRACT** — code is good enough to adapt/extract with attribution
- **REFERENCE** — useful to study for design decisions, but rewrite from scratch
- **IGNORE** — not relevant or poor quality, don't bother

Components to rate:

- [ ] Core data model types
- [ ] Relationship/edge model
- [ ] DSL parser
- [ ] SVG renderer
- [ ] Layout engine
- [ ] Mermaid exporter
- [ ] Structurizr JSON exporter
- [ ] Other exporters (PlantUML, D2, DOT)
- [ ] Web server
- [ ] CLI structure

### Key Takeaways

1. What are the best ideas in this codebase that we should steal (conceptually)?
2. What are the biggest mistakes we should avoid?
3. Are there any dependency choices they made that we should adopt or explicitly reject?
4. How does their data model compare to the dendryform schema in our design sketch? What's better about theirs, what's better about ours?

### Overall Assessment

- Is this a well-engineered Rust project, or does it feel AI-generated and lightly reviewed?
- On a scale of 1-10, how much value does this repo provide to dendryform's development?
- Estimated time saved (if any) by studying this repo vs. building from scratch?

## Output Format

Please produce a single markdown document with clear sections matching the steps above. Be specific and cite file paths and line numbers where relevant. Be honest — if the code is bad, say so clearly and explain why. If it's good, give credit where it's due.
