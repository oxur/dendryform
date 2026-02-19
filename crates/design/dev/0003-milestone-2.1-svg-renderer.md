# Milestone 2.1 — SVG Renderer

## Context

Phase 1 is complete: YAML → parse → layout → HTML pipeline works end-to-end. Phase 2 starts with the SVG renderer — same `LayoutPlan` + `Theme` input, but output is a self-contained static SVG with absolute pixel coordinates instead of CSS Grid.

## Design Decisions

1. **Direct string building** via `std::fmt::Write` — matches the HTML renderer pattern exactly
2. **Coordinate resolution in SVG crate** — the layout crate outputs relative grid indices; converting to absolute pixels is SVG-specific (HTML uses CSS Grid). Extraction to shared code can happen later if needed.
3. **Google Fonts `@import`** in SVG `<style>` with system font fallbacks — keeps SVG small; base64 WOFF2 embedding (~200KB) deferred to a later flag
4. **Monospace metrics** — `char_width = font_size * 0.6` for JetBrains Mono; sufficient for fixed-width text measurement without a font renderer
5. **No new dependencies** — just `dendryform-core`, `dendryform-layout`, `thiserror` (already in Cargo.toml); add `dendryform-parse` as dev-dep for tests

## File Structure

```
crates/dendryform-svg/src/
  lib.rs       — Public API: render_svg(), version(), module declarations
  error.rs     — SvgError (Fmt variant, mirrors HTML crate)
  escape.rs    — XML text escaping (& < > " ')
  metrics.rs   — Text measurement fns + SvgMetrics struct with all spacing constants
  resolve.rs   — Grid-to-pixel coordinate resolver: LayoutPlan → ResolvedPlan
  defs.rs      — SVG <defs>: font @import, arrowhead <marker>, connector <linearGradient>
  render.rs    — SVG string generation from ResolvedPlan
```

## Implementation Steps

### Step 1: Scaffold + error + escape + metrics
- `error.rs`: `SvgError` with `Fmt(std::fmt::Error)` variant (copy HTML pattern)
- `escape.rs`: `escape_xml()` — same as HTML `escape_html()` plus `'` → `&apos;`
- `metrics.rs`: `SvgMetrics` struct with all spacing constants (drawn from CSS in `css.rs`):
  - Title: 28px, subtitle: 13px, header margin: 56px
  - Tier label: 10px uppercase, margin: 10px
  - Node: 20px/16px padding, 14px title, 12px desc, 10px tech, 2px accent bar
  - Grid gap: 12px
  - Connector: 28px tall, 2px wide, 4px margin
  - Dots: 5 dots × 3px, 4px gap, 10px margin
  - Container: 24px padding (solid), 18px (dashed), 14px radius
  - Flow labels: 9px, 48px gap
  - Legend: 10×10 swatches, 24px item gap, 40px top margin
- `lib.rs`: Module declarations, re-export `SvgError` and `render_svg`
- `Cargo.toml`: Add `dendryform-parse` dev-dep

### Step 2: Coordinate resolver (`resolve.rs`)
Key types:
- `Rect { x, y, w, h }` — absolute pixel rectangle
- `ResolvedNode` — `&Node` + `Rect` + `Color`
- `ResolvedTier` — `Rect` + label position + `Vec<ResolvedNode>` + optional `ResolvedContainer`
- `ResolvedConnector` — center position, height, style, label
- `ResolvedContainer` — `Rect` + border type + label + `Vec<ResolvedLayer>`
- `ResolvedPlan` — total width/height + header + layers + legend

Algorithm: Walk layers top-to-bottom with a y-cursor:
1. Header: center title/subtitle, advance cursor by height + 56px margin
2. For each tier: compute cell width = (content_width - gaps) / columns, node height from content, place nodes in grid
3. For connectors: centered 2px line, advance 28px + margins
4. For containers: add padding, recurse into nested layers, compute bounding rect
5. Legend: center swatches horizontally
6. Total height = final y-cursor + bottom padding

Node height = 2px accent bar + 16px top pad + title line (~20px) + 4px gap + desc line (~18px) + optional tech row + 16px bottom pad

### Step 3: SVG defs (`defs.rs`)
- `<style>` with `@import url(...)` for JetBrains Mono + DM Sans
- `<marker id="arrowhead">` — small triangle polygon using border color
- `<linearGradient id="connector-grad">` — from border-highlight to border-normal (vertical)

### Step 4: Main renderer (`render.rs`)
- `render_svg(plan: &LayoutPlan, theme: &Theme, width: f32) -> Result<String, SvgError>`
- Call `resolve()` to get absolute coordinates, then emit SVG elements:
  - `write_svg_open()` — `<svg xmlns=... viewBox=...>` + background rect
  - `write_defs()` — from defs module
  - `write_header()` — `<text>` with `<tspan>` for accent
  - `write_layer()` — dispatch to tier/connector/flow labels
  - `write_tier()` — tier label text + node cards
  - `write_node()` — `<g>` with card rect (rx=10), accent bar rect, icon+title text, desc text, tech badges
  - `write_connector()` — `<line>` with `marker-end="url(#arrowhead)"` or 5 `<circle>` dots
  - `write_container()` — `<rect>` (stroke-dasharray for dashed), floating label, recursive layers
  - `write_flow_labels()` — centered `<text>` with ↓ arrow
  - `write_legend()` — swatch `<rect>` + label `<text>`

Key SVG notes:
- Tier labels: call `.to_uppercase()` in Rust (SVG lacks CSS `text-transform`)
- Accent top-bar: draw 2px rect with rx=10 at y=0 (bottom rounding hidden at 2px height)
- rgba colors: pass through as-is (modern browser SVG supports them)

### Step 5: Integration tests
- `test_render_taproot_svg` — parse taproot.yml → layout → render SVG → assert structural content
- `test_render_ai_kasu_svg` — same for ai-kasu
- `test_render_oxur_lisp_svg` — same for oxur-lisp
- Assert: contains `<svg`, node titles, connector markers, legend swatches, container elements

### Step 6: Wire into CLI
- Add `-f svg` format option to `dendryform render` command
- Add `--width` flag (default 1100) for SVG viewport width
- Update `dendryform-cli/Cargo.toml` to depend on `dendryform-svg`

## Key Files to Reference

- `crates/dendryform-html/src/render.rs` — rendering function decomposition pattern
- `crates/dendryform-html/src/css.rs` — all spacing/sizing constants (source of truth)
- `crates/dendryform-layout/src/geometry.rs` — input types (LayoutPlan, all geometry types)
- `crates/dendryform-core/src/theme.rs` — Theme API for colors, fonts, spacing
- `crates/dendryform-html/src/error.rs` — error type pattern to mirror

## Verification

1. `cargo test -p dendryform-svg` — all unit + integration tests pass
2. `cargo clippy --workspace --all-targets` — zero warnings
3. `cargo fmt --all --check` — clean
4. `dendryform render examples/taproot/taproot.yml -f svg -o taproot.svg` — produces valid SVG
5. Open taproot.svg in browser — visually matches the HTML output at 1100px width
