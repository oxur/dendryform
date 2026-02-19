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
