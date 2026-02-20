//! CSS generation from a Theme.

use std::fmt::Write;

use dendryform_core::Theme;

use crate::error::RenderError;

/// Generates the complete CSS stylesheet from a theme.
pub fn generate_css(theme: &Theme) -> Result<String, RenderError> {
    let mut css = String::with_capacity(4096);

    write_css_variables(&mut css, theme)?;
    write_base_styles(&mut css)?;
    write_header_styles(&mut css, theme)?;
    write_tier_styles(&mut css)?;
    write_connector_styles(&mut css)?;
    write_node_styles(&mut css, theme)?;
    write_color_variants(&mut css, theme)?;
    write_single_node_styles(&mut css)?;
    write_container_styles(&mut css, theme)?;
    write_grid_styles(&mut css)?;
    write_internal_connector_styles(&mut css, theme)?;
    write_flow_label_styles(&mut css, theme)?;
    write_legend_styles(&mut css, theme)?;
    write_animations(&mut css, theme)?;
    write_responsive(&mut css)?;

    Ok(css)
}

fn write_css_variables(css: &mut String, theme: &Theme) -> Result<(), RenderError> {
    let bg = theme.backgrounds();
    let text = theme.text();
    let borders = theme.borders();
    let sp = theme.spacing();

    writeln!(css, "  :root {{")?;
    writeln!(css, "    --bg: {};", bg.page())?;
    writeln!(css, "    --bg-card: {};", bg.card())?;
    writeln!(css, "    --bg-card-hover: {};", bg.card_hover())?;
    writeln!(css, "    --border: {};", borders.normal())?;
    writeln!(css, "    --border-highlight: {};", borders.highlight())?;
    writeln!(css, "    --text: {};", text.primary())?;
    writeln!(css, "    --text-dim: {};", text.dim())?;
    writeln!(css, "    --text-bright: {};", text.bright())?;

    for (color, set) in theme.palette().iter() {
        writeln!(css, "    --accent-{color}: {};", set.accent())?;
        writeln!(css, "    --accent-{color}-dim: {};", set.dim())?;
    }

    writeln!(css, "    --radius: {}px;", sp.radius())?;
    writeln!(css, "    --radius-sm: {}px;", sp.radius_sm())?;
    writeln!(css, "    --shadow: {};", sp.shadow())?;
    writeln!(css, "  }}")?;
    Ok(())
}

fn write_base_styles(css: &mut String) -> Result<(), RenderError> {
    writeln!(
        css,
        "  * {{ margin: 0; padding: 0; box-sizing: border-box; }}"
    )?;
    writeln!(css)?;
    writeln!(
        css,
        "  body {{ background: var(--bg); color: var(--text); font-family: 'DM Sans', sans-serif; min-height: 100vh; overflow-x: hidden; }}"
    )?;
    writeln!(css)?;
    writeln!(
        css,
        "  .canvas {{ max-width: 1100px; margin: 0 auto; padding: 48px 32px 64px; position: relative; }}"
    )?;
    Ok(())
}

fn write_header_styles(css: &mut String, _theme: &Theme) -> Result<(), RenderError> {
    writeln!(css)?;
    writeln!(
        css,
        "  .header {{ text-align: center; margin-bottom: 56px; animation: fadeIn 0.6s ease-out; }}"
    )?;
    writeln!(
        css,
        "  .header h1 {{ font-family: 'JetBrains Mono', monospace; font-weight: 600; font-size: 28px; color: var(--text-bright); letter-spacing: -0.5px; margin-bottom: 8px; }}"
    )?;
    writeln!(css, "  .header h1 span {{ color: var(--accent-green); }}")?;
    writeln!(
        css,
        "  .header .subtitle {{ font-size: 13px; color: var(--text-dim); font-family: 'JetBrains Mono', monospace; font-weight: 300; }}"
    )?;
    Ok(())
}

fn write_tier_styles(css: &mut String) -> Result<(), RenderError> {
    writeln!(css)?;
    writeln!(
        css,
        "  .tier {{ position: relative; margin-bottom: 20px; animation: slideUp 0.5s ease-out both; }}"
    )?;
    for i in 2..=6 {
        writeln!(
            css,
            "  .tier:nth-child({i}) {{ animation-delay: {}s; }}",
            (i - 1) as f32 * 0.1
        )?;
    }
    writeln!(css)?;
    writeln!(
        css,
        "  .tier-label {{ font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500; text-transform: uppercase; letter-spacing: 2px; color: var(--text-dim); margin-bottom: 10px; padding-left: 4px; }}"
    )?;
    Ok(())
}

fn write_connector_styles(css: &mut String) -> Result<(), RenderError> {
    writeln!(css)?;
    writeln!(
        css,
        "  .connector {{ display: flex; justify-content: center; margin: 4px 0; position: relative; }}"
    )?;
    writeln!(
        css,
        "  .connector .line {{ width: 2px; height: 28px; background: linear-gradient(to bottom, var(--border-highlight), var(--border)); position: relative; }}"
    )?;
    writeln!(
        css,
        "  .connector .line::after {{ content: ''; position: absolute; bottom: -3px; left: 50%; transform: translateX(-50%); width: 0; height: 0; border-left: 4px solid transparent; border-right: 4px solid transparent; border-top: 5px solid var(--border); }}"
    )?;
    writeln!(
        css,
        "  .connector .protocol-label {{ position: absolute; right: calc(50% + 16px); top: 50%; transform: translateY(-50%); font-family: 'JetBrains Mono', monospace; font-size: 10px; color: var(--text-dim); white-space: nowrap; }}"
    )?;
    Ok(())
}

fn write_node_styles(css: &mut String, _theme: &Theme) -> Result<(), RenderError> {
    writeln!(css)?;
    writeln!(
        css,
        "  .node {{ background: var(--bg-card); border: 1px solid var(--border); border-radius: var(--radius); padding: 16px 20px; transition: all 0.25s ease; cursor: default; position: relative; overflow: hidden; }}"
    )?;
    writeln!(
        css,
        "  .node:hover {{ background: var(--bg-card-hover); border-color: var(--border-highlight); box-shadow: var(--shadow); transform: translateY(-1px); }}"
    )?;
    writeln!(
        css,
        "  .node::before {{ content: ''; position: absolute; top: 0; left: 0; right: 0; height: 2px; border-radius: var(--radius) var(--radius) 0 0; }}"
    )?;
    writeln!(
        css,
        "  .node .node-title {{ font-family: 'JetBrains Mono', monospace; font-weight: 500; font-size: 14px; color: var(--text-bright); margin-bottom: 4px; display: flex; align-items: center; gap: 8px; }}"
    )?;
    writeln!(
        css,
        "  .node .node-title .icon {{ font-size: 15px; line-height: 1; }}"
    )?;
    writeln!(
        css,
        "  .node .node-desc {{ font-size: 12px; color: var(--text-dim); line-height: 1.5; }}"
    )?;
    writeln!(
        css,
        "  .node .node-tech {{ font-family: 'JetBrains Mono', monospace; font-size: 10px; color: var(--text-dim); margin-top: 8px; display: flex; flex-wrap: wrap; gap: 6px; }}"
    )?;
    writeln!(
        css,
        "  .node .node-tech span {{ background: rgba(255,255,255,0.04); padding: 2px 8px; border-radius: 4px; border: 1px solid rgba(255,255,255,0.06); }}"
    )?;
    Ok(())
}

fn write_color_variants(css: &mut String, theme: &Theme) -> Result<(), RenderError> {
    writeln!(css)?;
    for (color, set) in theme.palette().iter() {
        writeln!(
            css,
            "  .node.{color}::before {{ background: var(--accent-{color}); }}"
        )?;
        writeln!(
            css,
            "  .node.{color}:hover {{ border-color: {}; }}",
            set.dim()
        )?;
        writeln!(
            css,
            "  .node.{color} .node-title .icon {{ color: var(--accent-{color}); }}"
        )?;
    }
    Ok(())
}

fn write_single_node_styles(css: &mut String) -> Result<(), RenderError> {
    writeln!(css)?;
    writeln!(
        css,
        "  .client-node {{ text-align: center; padding: 20px; }}"
    )?;
    writeln!(
        css,
        "  .client-node .node-title {{ justify-content: center; }}"
    )?;
    Ok(())
}

fn write_container_styles(css: &mut String, _theme: &Theme) -> Result<(), RenderError> {
    writeln!(css)?;
    writeln!(
        css,
        "  .container-solid {{ background: rgba(17, 24, 32, 0.5); border: 1px solid var(--border); border-radius: 14px; padding: 24px; position: relative; }}"
    )?;
    writeln!(
        css,
        "  .container-solid > .container-label {{ position: absolute; top: -10px; left: 24px; font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500; text-transform: uppercase; letter-spacing: 2px; background: var(--bg); padding: 0 10px; }}"
    )?;
    writeln!(css)?;
    writeln!(
        css,
        "  .container-dashed {{ border: 1px dashed rgba(255,255,255,0.1); border-radius: var(--radius); padding: 18px; margin-bottom: 16px; position: relative; }}"
    )?;
    writeln!(
        css,
        "  .container-dashed > .container-label {{ position: absolute; top: -9px; left: 18px; font-family: 'JetBrains Mono', monospace; font-size: 9px; font-weight: 500; text-transform: uppercase; letter-spacing: 1.5px; background: var(--bg-card); padding: 0 8px; }}"
    )?;
    Ok(())
}

fn write_grid_styles(css: &mut String) -> Result<(), RenderError> {
    writeln!(css)?;
    for n in 1..=4 {
        writeln!(
            css,
            "  .grid-{n} {{ display: grid; grid-template-columns: repeat({n}, 1fr); gap: 12px; margin-bottom: 16px; }}"
        )?;
    }
    Ok(())
}

fn write_internal_connector_styles(css: &mut String, _theme: &Theme) -> Result<(), RenderError> {
    writeln!(css)?;
    writeln!(
        css,
        "  .internal-connector {{ display: flex; justify-content: center; margin: 10px 0; }}"
    )?;
    writeln!(
        css,
        "  .internal-connector .dots {{ display: flex; gap: 4px; align-items: center; }}"
    )?;
    writeln!(
        css,
        "  .internal-connector .dot {{ width: 3px; height: 3px; border-radius: 50%; background: var(--border-highlight); }}"
    )?;
    Ok(())
}

fn write_flow_label_styles(css: &mut String, _theme: &Theme) -> Result<(), RenderError> {
    writeln!(css)?;
    writeln!(
        css,
        "  .flow-labels {{ display: flex; justify-content: center; gap: 48px; margin: 8px 0 4px; }}"
    )?;
    writeln!(
        css,
        "  .flow-label {{ font-family: 'JetBrains Mono', monospace; font-size: 9px; color: var(--text-dim); display: flex; align-items: center; gap: 5px; }}"
    )?;
    writeln!(
        css,
        "  .flow-label .arrow {{ color: var(--border-highlight); }}"
    )?;
    Ok(())
}

fn write_legend_styles(css: &mut String, theme: &Theme) -> Result<(), RenderError> {
    writeln!(css)?;
    writeln!(
        css,
        "  .legend {{ margin-top: 40px; display: flex; justify-content: center; gap: 24px; flex-wrap: wrap; }}"
    )?;
    writeln!(
        css,
        "  .legend-item {{ display: flex; align-items: center; gap: 6px; font-family: 'JetBrains Mono', monospace; font-size: 10px; color: var(--text-dim); }}"
    )?;
    writeln!(
        css,
        "  .legend-item .swatch {{ width: 10px; height: 10px; border-radius: 3px; }}"
    )?;
    for (color, _) in theme.palette().iter() {
        writeln!(
            css,
            "  .swatch.{color} {{ background: var(--accent-{color}); }}"
        )?;
    }
    Ok(())
}

fn write_animations(css: &mut String, theme: &Theme) -> Result<(), RenderError> {
    if !theme.animate() {
        writeln!(css)?;
        writeln!(
            css,
            "  *, *::before, *::after {{ animation: none !important; transition: none !important; }}"
        )?;
        return Ok(());
    }
    writeln!(css)?;
    writeln!(
        css,
        "  @keyframes fadeIn {{ from {{ opacity: 0; }} to {{ opacity: 1; }} }}"
    )?;
    writeln!(
        css,
        "  @keyframes slideUp {{ from {{ opacity: 0; transform: translateY(16px); }} to {{ opacity: 1; transform: translateY(0); }} }}"
    )?;
    Ok(())
}

fn write_responsive(css: &mut String) -> Result<(), RenderError> {
    writeln!(css)?;
    writeln!(css, "  @media (max-width: 800px) {{")?;
    writeln!(
        css,
        "    .grid-4 {{ grid-template-columns: repeat(2, 1fr); }}"
    )?;
    writeln!(css, "    .grid-3 {{ grid-template-columns: 1fr; }}")?;
    writeln!(css, "    .canvas {{ padding: 24px 16px; }}")?;
    writeln!(css, "    .flow-labels {{ gap: 16px; flex-wrap: wrap; }}")?;
    writeln!(css, "  }}")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dendryform_core::Theme;

    fn dark_theme() -> Theme {
        Theme::dark()
    }

    #[test]
    fn test_generate_css_ok() {
        let theme = dark_theme();
        let css = generate_css(&theme).unwrap();
        assert!(!css.is_empty());
    }

    #[test]
    fn test_css_variables() {
        let theme = dark_theme();
        let mut css = String::new();
        write_css_variables(&mut css, &theme).unwrap();
        assert!(css.contains(":root"));
        assert!(css.contains("--bg:"));
        assert!(css.contains("--bg-card:"));
        assert!(css.contains("--bg-card-hover:"));
        assert!(css.contains("--border:"));
        assert!(css.contains("--border-highlight:"));
        assert!(css.contains("--text:"));
        assert!(css.contains("--text-dim:"));
        assert!(css.contains("--text-bright:"));
        assert!(css.contains("--radius:"));
        assert!(css.contains("--radius-sm:"));
        assert!(css.contains("--shadow:"));
    }

    #[test]
    fn test_base_styles() {
        let mut css = String::new();
        write_base_styles(&mut css).unwrap();
        assert!(css.contains("body"));
        assert!(css.contains(".canvas"));
        assert!(css.contains("box-sizing: border-box"));
    }

    #[test]
    fn test_header_styles() {
        let theme = dark_theme();
        let mut css = String::new();
        write_header_styles(&mut css, &theme).unwrap();
        assert!(css.contains(".header"));
        assert!(css.contains("h1"));
        assert!(css.contains(".subtitle"));
    }

    #[test]
    fn test_tier_styles() {
        let mut css = String::new();
        write_tier_styles(&mut css).unwrap();
        assert!(css.contains(".tier"));
        assert!(css.contains("animation-delay"));
        assert!(css.contains(".tier-label"));
    }

    #[test]
    fn test_connector_styles() {
        let mut css = String::new();
        write_connector_styles(&mut css).unwrap();
        assert!(css.contains(".connector"));
        assert!(css.contains(".line"));
        assert!(css.contains(".protocol-label"));
    }

    #[test]
    fn test_node_styles() {
        let theme = dark_theme();
        let mut css = String::new();
        write_node_styles(&mut css, &theme).unwrap();
        assert!(css.contains(".node"));
        assert!(css.contains(".node-title"));
        assert!(css.contains(".node-desc"));
        assert!(css.contains(".node-tech"));
    }

    #[test]
    fn test_color_variants() {
        let theme = dark_theme();
        let mut css = String::new();
        write_color_variants(&mut css, &theme).unwrap();
        assert!(css.contains("::before"));
        assert!(css.contains(":hover"));
    }

    #[test]
    fn test_single_node_styles() {
        let mut css = String::new();
        write_single_node_styles(&mut css).unwrap();
        assert!(css.contains(".client-node"));
    }

    #[test]
    fn test_container_styles() {
        let theme = dark_theme();
        let mut css = String::new();
        write_container_styles(&mut css, &theme).unwrap();
        assert!(css.contains(".container-solid"));
        assert!(css.contains(".container-dashed"));
        assert!(css.contains(".container-label"));
    }

    #[test]
    fn test_grid_styles() {
        let mut css = String::new();
        write_grid_styles(&mut css).unwrap();
        assert!(css.contains(".grid-1"));
        assert!(css.contains(".grid-2"));
        assert!(css.contains(".grid-3"));
        assert!(css.contains(".grid-4"));
    }

    #[test]
    fn test_internal_connector_styles() {
        let theme = dark_theme();
        let mut css = String::new();
        write_internal_connector_styles(&mut css, &theme).unwrap();
        assert!(css.contains(".internal-connector"));
        assert!(css.contains(".dots"));
        assert!(css.contains(".dot"));
    }

    #[test]
    fn test_flow_label_styles() {
        let theme = dark_theme();
        let mut css = String::new();
        write_flow_label_styles(&mut css, &theme).unwrap();
        assert!(css.contains(".flow-labels"));
        assert!(css.contains(".flow-label"));
        assert!(css.contains(".arrow"));
    }

    #[test]
    fn test_legend_styles() {
        let theme = dark_theme();
        let mut css = String::new();
        write_legend_styles(&mut css, &theme).unwrap();
        assert!(css.contains(".legend"));
        assert!(css.contains(".legend-item"));
        assert!(css.contains(".swatch"));
    }

    #[test]
    fn test_animations_enabled() {
        let theme = dark_theme();
        let mut css = String::new();
        write_animations(&mut css, &theme).unwrap();
        assert!(css.contains("@keyframes fadeIn"));
        assert!(css.contains("@keyframes slideUp"));
    }

    #[test]
    fn test_animations_disabled() {
        let theme = dark_theme();
        let overrides = dendryform_core::ThemeOverrides {
            animate: Some(false),
            ..Default::default()
        };
        let theme = theme.merge(overrides);
        let mut css = String::new();
        write_animations(&mut css, &theme).unwrap();
        assert!(css.contains("animation: none !important"));
        assert!(!css.contains("@keyframes"));
    }

    #[test]
    fn test_responsive() {
        let mut css = String::new();
        write_responsive(&mut css).unwrap();
        assert!(css.contains("@media (max-width: 800px)"));
        assert!(css.contains(".grid-4"));
        assert!(css.contains(".grid-3"));
        assert!(css.contains(".canvas"));
        assert!(css.contains(".flow-labels"));
    }

    #[test]
    fn test_generate_css_contains_all_sections() {
        let theme = dark_theme();
        let css = generate_css(&theme).unwrap();

        // Verify CSS variables
        assert!(css.contains(":root"));
        assert!(css.contains("--bg:"));

        // Verify base styles
        assert!(css.contains("box-sizing: border-box"));
        assert!(css.contains("body"));

        // Verify header styles
        assert!(css.contains(".header"));
        assert!(css.contains(".subtitle"));

        // Verify tier styles
        assert!(css.contains(".tier"));
        assert!(css.contains(".tier-label"));

        // Verify connector styles
        assert!(css.contains(".connector"));
        assert!(css.contains(".protocol-label"));

        // Verify node styles
        assert!(css.contains(".node"));
        assert!(css.contains(".node-title"));
        assert!(css.contains(".node-desc"));
        assert!(css.contains(".node-tech"));

        // Verify color variant styles
        assert!(css.contains("::before"));

        // Verify single node styles
        assert!(css.contains(".client-node"));

        // Verify container styles
        assert!(css.contains(".container-solid"));
        assert!(css.contains(".container-dashed"));

        // Verify grid styles
        assert!(css.contains(".grid-1"));
        assert!(css.contains(".grid-4"));

        // Verify internal connector styles
        assert!(css.contains(".internal-connector"));
        assert!(css.contains(".dot"));

        // Verify flow label styles
        assert!(css.contains(".flow-labels"));
        assert!(css.contains(".flow-label"));

        // Verify legend styles
        assert!(css.contains(".legend"));
        assert!(css.contains(".swatch"));

        // Verify animations (enabled by default)
        assert!(css.contains("@keyframes fadeIn"));
        assert!(css.contains("@keyframes slideUp"));

        // Verify responsive
        assert!(css.contains("@media"));
    }

    #[test]
    fn test_css_variables_palette_entries() {
        let theme = dark_theme();
        let mut css = String::new();
        write_css_variables(&mut css, &theme).unwrap();

        // Verify palette variables exist (we may not know exact colors due to HashMap ordering)
        assert!(css.contains("--accent-"));
    }

    #[test]
    fn test_color_variants_hover_and_icon() {
        let theme = dark_theme();
        let mut css = String::new();
        write_color_variants(&mut css, &theme).unwrap();

        // Each palette color should generate three rules: ::before, :hover, .icon
        assert!(css.contains("::before"));
        assert!(css.contains(":hover"));
        assert!(css.contains(".icon"));
    }

    #[test]
    fn test_legend_styles_swatch_per_color() {
        let theme = dark_theme();
        let mut css = String::new();
        write_legend_styles(&mut css, &theme).unwrap();

        // Should have swatch rules for palette colors
        assert!(css.contains(".swatch."));
        assert!(css.contains("var(--accent-"));
    }

    #[test]
    fn test_tier_styles_animation_delays() {
        let mut css = String::new();
        write_tier_styles(&mut css).unwrap();

        // Should have animation delays for tiers 2-6
        assert!(css.contains("nth-child(2)"));
        assert!(css.contains("nth-child(6)"));
        // Check delay values
        assert!(css.contains("0.1s"));
        assert!(css.contains("0.5s"));
    }

    #[test]
    fn test_grid_styles_all_four_columns() {
        let mut css = String::new();
        write_grid_styles(&mut css).unwrap();

        // Verify repeat(N, 1fr) for all grid sizes
        assert!(css.contains("repeat(1, 1fr)"));
        assert!(css.contains("repeat(2, 1fr)"));
        assert!(css.contains("repeat(3, 1fr)"));
        assert!(css.contains("repeat(4, 1fr)"));
    }

    #[test]
    fn test_node_styles_all_sub_elements() {
        let theme = dark_theme();
        let mut css = String::new();
        write_node_styles(&mut css, &theme).unwrap();

        assert!(css.contains(".node:hover"));
        assert!(css.contains(".node::before"));
        assert!(css.contains(".node .node-title .icon"));
    }

    #[test]
    fn test_container_styles_both_variants() {
        let theme = dark_theme();
        let mut css = String::new();
        write_container_styles(&mut css, &theme).unwrap();

        assert!(css.contains(".container-solid > .container-label"));
        assert!(css.contains(".container-dashed > .container-label"));
    }
}
