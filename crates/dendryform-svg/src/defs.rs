//! SVG `<defs>` section: font imports, arrowhead marker, connector gradient.

use std::fmt::Write;

use dendryform_core::Theme;

use crate::error::SvgError;

/// Writes the `<defs>` block containing shared SVG definitions.
pub fn write_defs(svg: &mut String, theme: &Theme) -> Result<(), SvgError> {
    writeln!(svg, "  <defs>")?;
    write_font_style(svg, theme)?;
    write_arrowhead_marker(svg, theme)?;
    write_connector_gradient(svg, theme)?;
    writeln!(svg, "  </defs>")?;
    Ok(())
}

fn write_font_style(svg: &mut String, theme: &Theme) -> Result<(), SvgError> {
    let display_font = theme.fonts().display().replace(' ', "+");
    let body_font = theme.fonts().body().replace(' ', "+");

    writeln!(svg, "    <style>")?;
    writeln!(
        svg,
        "      @import url('https://fonts.googleapis.com/css2?family={display_font}:wght@300;400;500;600&amp;family={body_font}:wght@300;400;500;600;700&amp;display=swap');"
    )?;
    writeln!(svg, "    </style>")?;
    Ok(())
}

fn write_arrowhead_marker(svg: &mut String, theme: &Theme) -> Result<(), SvgError> {
    let border_color = theme.borders().normal();
    writeln!(
        svg,
        "    <marker id=\"arrowhead\" markerWidth=\"8\" markerHeight=\"6\" refX=\"4\" refY=\"3\" orient=\"auto\" markerUnits=\"userSpaceOnUse\">"
    )?;
    writeln!(
        svg,
        "      <polygon points=\"0 0, 8 3, 0 6\" fill=\"{border_color}\"/>"
    )?;
    writeln!(svg, "    </marker>")?;
    Ok(())
}

fn write_connector_gradient(svg: &mut String, theme: &Theme) -> Result<(), SvgError> {
    let highlight = theme.borders().highlight();
    let normal = theme.borders().normal();
    writeln!(
        svg,
        "    <linearGradient id=\"connector-grad\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\">"
    )?;
    writeln!(
        svg,
        "      <stop offset=\"0%\" stop-color=\"{highlight}\"/>"
    )?;
    writeln!(svg, "      <stop offset=\"100%\" stop-color=\"{normal}\"/>")?;
    writeln!(svg, "    </linearGradient>")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_defs_contains_style() {
        let theme = Theme::dark();
        let mut svg = String::new();
        write_defs(&mut svg, &theme).unwrap();

        assert!(svg.contains("<defs>"));
        assert!(svg.contains("</defs>"));
        assert!(svg.contains("@import url"));
        assert!(svg.contains("JetBrains+Mono"));
        assert!(svg.contains("DM+Sans"));
    }

    #[test]
    fn test_write_defs_contains_arrowhead() {
        let theme = Theme::dark();
        let mut svg = String::new();
        write_defs(&mut svg, &theme).unwrap();

        assert!(svg.contains("marker id=\"arrowhead\""));
        assert!(svg.contains("<polygon"));
    }

    #[test]
    fn test_write_defs_contains_gradient() {
        let theme = Theme::dark();
        let mut svg = String::new();
        write_defs(&mut svg, &theme).unwrap();

        assert!(svg.contains("linearGradient id=\"connector-grad\""));
        assert!(svg.contains(theme.borders().highlight()));
        assert!(svg.contains(theme.borders().normal()));
    }

    #[test]
    fn test_write_font_style_directly() {
        let theme = Theme::dark();
        let mut svg = String::new();
        write_font_style(&mut svg, &theme).unwrap();
        assert!(svg.contains("<style>"));
        assert!(svg.contains("</style>"));
        assert!(svg.contains("@import url"));
        assert!(svg.contains("JetBrains+Mono"));
        assert!(svg.contains("DM+Sans"));
    }

    #[test]
    fn test_write_arrowhead_marker_directly() {
        let theme = Theme::dark();
        let mut svg = String::new();
        write_arrowhead_marker(&mut svg, &theme).unwrap();
        assert!(svg.contains("<marker"));
        assert!(svg.contains("id=\"arrowhead\""));
        assert!(svg.contains("<polygon"));
        assert!(svg.contains("</marker>"));
    }

    #[test]
    fn test_write_connector_gradient_directly() {
        let theme = Theme::dark();
        let mut svg = String::new();
        write_connector_gradient(&mut svg, &theme).unwrap();
        assert!(svg.contains("<linearGradient"));
        assert!(svg.contains("</linearGradient>"));
        assert!(svg.contains("stop offset=\"0%\""));
        assert!(svg.contains("stop offset=\"100%\""));
    }

    #[test]
    fn test_write_defs_arrowhead_uses_border_color() {
        let theme = Theme::dark();
        let mut svg = String::new();
        write_defs(&mut svg, &theme).unwrap();

        // Arrowhead fill should use the normal border color
        let border_color = theme.borders().normal();
        assert!(svg.contains(&format!("fill=\"{border_color}\"")));
    }

    #[test]
    fn test_write_defs_gradient_uses_theme_colors() {
        let theme = Theme::dark();
        let mut svg = String::new();
        write_defs(&mut svg, &theme).unwrap();

        let highlight = theme.borders().highlight();
        let normal = theme.borders().normal();
        assert!(svg.contains(&format!("stop-color=\"{highlight}\"")));
        assert!(svg.contains(&format!("stop-color=\"{normal}\"")));
    }

    #[test]
    fn test_write_font_style_has_amp_escaped() {
        let theme = Theme::dark();
        let mut svg = String::new();
        write_font_style(&mut svg, &theme).unwrap();

        // In SVG XML, & should be &amp; in URLs
        assert!(svg.contains("&amp;"));
    }

    #[test]
    fn test_write_defs_marker_attributes() {
        let theme = Theme::dark();
        let mut svg = String::new();
        write_arrowhead_marker(&mut svg, &theme).unwrap();

        assert!(svg.contains("markerWidth=\"8\""));
        assert!(svg.contains("markerHeight=\"6\""));
        assert!(svg.contains("refX=\"4\""));
        assert!(svg.contains("refY=\"3\""));
        assert!(svg.contains("orient=\"auto\""));
    }
}
