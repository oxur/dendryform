//! # dendryform-png
//!
//! PNG renderer for dendryform diagrams.
//!
//! Wraps `dendryform-svg` output and rasterizes it to PNG using
//! `resvg`. This is a thin adapter — all layout and visual logic
//! lives in the SVG crate.
//!
//! ## Quick Start
//!
//! ```no_run
//! use dendryform_core::Theme;
//! use dendryform_layout::compute_layout;
//! use dendryform_svg::render_svg;
//! use dendryform_png::render_png;
//!
//! let diagram = dendryform_parse::parse_yaml_file("examples/taproot/architecture.yaml").unwrap();
//! let plan = compute_layout(&diagram).unwrap();
//! let svg = render_svg(&plan, &Theme::dark(), 1100.0).unwrap();
//! let png = render_png(&svg, 1.0).unwrap();
//! std::fs::write("output.png", png).unwrap();
//! ```

mod error;
mod fonts;

pub use error::PngError;
pub use fonts::create_fontdb;

use dendryform_core::Theme;
use dendryform_layout::LayoutPlan;
use dendryform_svg::render_svg;
use resvg::tiny_skia;
use resvg::usvg;

/// Returns the version of the dendryform-png crate.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Rasterizes an SVG string to PNG bytes.
///
/// `scale` is a multiplier applied to the SVG dimensions (1.0 = 1x, 2.0 = retina).
pub fn render_png(svg: &str, scale: f32) -> Result<Vec<u8>, PngError> {
    let fontdb = create_fontdb();

    let opts = usvg::Options {
        fontdb,
        ..Default::default()
    };

    let tree = usvg::Tree::from_str(svg, &opts).map_err(|e| PngError::Parse(e.to_string()))?;

    let size = tree.size();
    let w = (size.width() * scale).ceil() as u32;
    let h = (size.height() * scale).ceil() as u32;

    let mut pixmap = tiny_skia::Pixmap::new(w, h).ok_or(PngError::Pixmap)?;

    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(scale, scale),
        &mut pixmap.as_mut(),
    );

    pixmap
        .encode_png()
        .map_err(|e| PngError::Encode(e.to_string()))
}

/// Convenience function: render a layout plan to PNG bytes.
///
/// Calls `render_svg()` then `render_png()`.
pub fn render_diagram_png(
    plan: &LayoutPlan<'_>,
    theme: &Theme,
    width: f32,
    scale: f32,
) -> Result<Vec<u8>, PngError> {
    let svg = render_svg(plan, theme, width)?;
    render_png(&svg, scale)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="80"><rect width="100" height="80" fill="#0a0e14"/></svg>"##;

    #[test]
    fn test_version_is_set() {
        assert_eq!(version(), "0.1.0");
    }

    #[test]
    fn test_render_png_simple() {
        let png = render_png(MINIMAL_SVG, 1.0).unwrap();
        assert!(!png.is_empty());
        // PNG magic bytes
        assert_eq!(&png[0..4], &[137, 80, 78, 71]);
    }

    #[test]
    fn test_render_png_scale_2x() {
        let png_1x = render_png(MINIMAL_SVG, 1.0).unwrap();
        let png_2x = render_png(MINIMAL_SVG, 2.0).unwrap();
        // 2x should produce more bytes (larger image)
        assert!(png_2x.len() > png_1x.len());
        // Both should be valid PNGs
        assert_eq!(&png_2x[0..4], &[137, 80, 78, 71]);
    }

    #[test]
    fn test_render_png_invalid_svg() {
        let result = render_png("not valid svg at all", 1.0);
        assert!(result.is_err());
        match result.unwrap_err() {
            PngError::Parse(_) => {}
            other => panic!("expected Parse error, got: {other:?}"),
        }
    }

    #[test]
    fn test_render_png_zero_dimensions() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="0" height="0"></svg>"#;
        let result = render_png(svg, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_render_taproot_png() {
        let yaml = include_str!("../../../examples/taproot/architecture.yaml");
        let diagram = dendryform_parse::parse_yaml(yaml).unwrap();
        let plan = dendryform_layout::compute_layout(&diagram).unwrap();
        let svg = render_svg(&plan, &Theme::dark(), 1100.0).unwrap();
        let png = render_png(&svg, 1.0).unwrap();
        assert!(!png.is_empty());
        assert_eq!(&png[0..4], &[137, 80, 78, 71]);
    }

    #[test]
    fn test_render_ai_kasu_png() {
        let yaml = include_str!("../../../examples/ai-kasu/architecture.yaml");
        let diagram = dendryform_parse::parse_yaml(yaml).unwrap();
        let plan = dendryform_layout::compute_layout(&diagram).unwrap();
        let svg = render_svg(&plan, &Theme::dark(), 1100.0).unwrap();
        let png = render_png(&svg, 1.0).unwrap();
        assert!(!png.is_empty());
        assert_eq!(&png[0..4], &[137, 80, 78, 71]);
    }

    #[test]
    fn test_render_oxur_lisp_png() {
        let yaml = include_str!("../../../examples/oxur-lisp/architecture.yaml");
        let diagram = dendryform_parse::parse_yaml(yaml).unwrap();
        let plan = dendryform_layout::compute_layout(&diagram).unwrap();
        let svg = render_svg(&plan, &Theme::dark(), 1100.0).unwrap();
        let png = render_png(&svg, 1.0).unwrap();
        assert!(!png.is_empty());
        assert_eq!(&png[0..4], &[137, 80, 78, 71]);
    }

    #[test]
    fn test_render_diagram_png_convenience() {
        let yaml = include_str!("../../../examples/taproot/architecture.yaml");
        let diagram = dendryform_parse::parse_yaml(yaml).unwrap();
        let plan = dendryform_layout::compute_layout(&diagram).unwrap();
        let png = render_diagram_png(&plan, &Theme::dark(), 1100.0, 1.0).unwrap();
        assert!(!png.is_empty());
        assert_eq!(&png[0..4], &[137, 80, 78, 71]);
    }
}
