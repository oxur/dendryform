//! HTML rendering — transforms a LayoutPlan + Theme into self-contained HTML.

use std::fmt::Write;

use dendryform_core::Theme;
use dendryform_layout::{
    ConnectorGeometry, ContainerGeometry, FlowLabelsGeometry, LayerGeometry, LayoutPlan,
    NodeGeometry, TierGeometry,
};

use crate::css::generate_css;
use crate::error::RenderError;

/// Renders a layout plan and theme into a self-contained HTML string.
pub fn render_html(plan: &LayoutPlan<'_>, theme: &Theme) -> Result<String, RenderError> {
    let mut html = String::with_capacity(16384);

    write_document_head(&mut html, plan, theme)?;
    write_body(&mut html, plan, theme)?;

    Ok(html)
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn write_document_head(
    html: &mut String,
    plan: &LayoutPlan<'_>,
    theme: &Theme,
) -> Result<(), RenderError> {
    writeln!(html, "<!DOCTYPE html>")?;
    writeln!(html, "<html lang=\"en\">")?;
    writeln!(html, "<head>")?;
    writeln!(html, "<meta charset=\"UTF-8\">")?;
    writeln!(
        html,
        "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">"
    )?;
    writeln!(
        html,
        "<title>{} \u{00b7} {}</title>",
        escape_html(&plan.header.title_accent),
        escape_html(&plan.header.title_text),
    )?;

    // Font import
    let display_font = theme.fonts().display().replace(' ', "+");
    let body_font = theme.fonts().body().replace(' ', "+");
    writeln!(
        html,
        "<link rel=\"stylesheet\" href=\"https://fonts.googleapis.com/css2?family={display_font}:wght@300;400;500;600&family={body_font}:wght@300;400;500;600;700&display=swap\">"
    )?;

    writeln!(html, "<style>")?;
    let css = generate_css(theme)?;
    write!(html, "{css}")?;
    writeln!(html, "</style>")?;
    writeln!(html, "</head>")?;

    Ok(())
}

fn write_body(html: &mut String, plan: &LayoutPlan<'_>, _theme: &Theme) -> Result<(), RenderError> {
    writeln!(html, "<body>")?;
    writeln!(html, "<div class=\"canvas\">")?;

    // Header
    write_header(html, plan)?;

    // Layers
    for layer in &plan.layers {
        write_layer(html, layer)?;
    }

    // Legend
    write_legend(html, plan)?;

    writeln!(html, "</div>")?;
    writeln!(html, "</body>")?;
    writeln!(html, "</html>")?;

    Ok(())
}

fn write_header(html: &mut String, plan: &LayoutPlan<'_>) -> Result<(), RenderError> {
    writeln!(html, "  <div class=\"header\">")?;
    writeln!(
        html,
        "    <h1><span>{}</span> \u{00b7} {}</h1>",
        escape_html(&plan.header.title_accent),
        escape_html(&plan.header.title_text),
    )?;
    writeln!(
        html,
        "    <div class=\"subtitle\">{}</div>",
        escape_html(&plan.header.subtitle),
    )?;
    writeln!(html, "  </div>")?;
    Ok(())
}

fn write_layer(html: &mut String, layer: &LayerGeometry<'_>) -> Result<(), RenderError> {
    match layer {
        LayerGeometry::Tier(tier) => write_tier(html, tier),
        LayerGeometry::Connector(conn) => write_connector(html, conn),
        LayerGeometry::FlowLabels(labels) => write_flow_labels(html, labels),
    }
}

fn write_tier(html: &mut String, tier: &TierGeometry<'_>) -> Result<(), RenderError> {
    writeln!(html, "  <div class=\"tier\">")?;

    if let Some(container) = &tier.container {
        write_container(html, container, tier.label.as_deref())?;
    } else {
        if let Some(label) = &tier.label {
            writeln!(
                html,
                "    <div class=\"tier-label\">{}</div>",
                escape_html(label)
            )?;
        }
        write_node_grid(html, &tier.nodes, tier.columns, tier.columns == 1)?;
    }

    writeln!(html, "  </div>")?;
    Ok(())
}

fn write_node_grid(
    html: &mut String,
    nodes: &[NodeGeometry<'_>],
    columns: usize,
    is_single: bool,
) -> Result<(), RenderError> {
    if nodes.is_empty() {
        return Ok(());
    }

    writeln!(html, "    <div class=\"grid-{columns}\">")?;
    for ng in nodes {
        write_node(html, ng, is_single)?;
    }
    writeln!(html, "    </div>")?;
    Ok(())
}

fn write_node(
    html: &mut String,
    ng: &NodeGeometry<'_>,
    is_single: bool,
) -> Result<(), RenderError> {
    let node = ng.node;
    let color = node.color();
    let single_class = if is_single { " client-node" } else { "" };

    writeln!(html, "      <div class=\"node {color}{single_class}\">")?;
    writeln!(
        html,
        "        <div class=\"node-title\"><span class=\"icon\">{}</span> {}</div>",
        escape_html(node.icon()),
        escape_html(node.title()),
    )?;
    writeln!(
        html,
        "        <div class=\"node-desc\">{}</div>",
        escape_html(node.description()),
    )?;

    let tech = node.tech();
    if !tech.is_empty() {
        write!(html, "        <div class=\"node-tech\">")?;
        for t in tech {
            write!(html, "<span>{}</span>", escape_html(&t.to_string()))?;
        }
        writeln!(html, "</div>")?;
    }

    writeln!(html, "      </div>")?;
    Ok(())
}

fn write_connector(html: &mut String, conn: &ConnectorGeometry) -> Result<(), RenderError> {
    if conn.is_internal {
        writeln!(html, "    <div class=\"internal-connector\">")?;
        writeln!(html, "      <div class=\"dots\">")?;
        for _ in 0..5 {
            write!(html, "        <div class=\"dot\"></div>")?;
        }
        writeln!(html)?;
        writeln!(html, "      </div>")?;
        writeln!(html, "    </div>")?;
    } else {
        writeln!(html, "  <div class=\"connector\">")?;
        writeln!(html, "    <div class=\"line\"></div>")?;
        if let Some(label) = &conn.label {
            writeln!(
                html,
                "    <div class=\"protocol-label\">{}</div>",
                escape_html(label),
            )?;
        }
        writeln!(html, "  </div>")?;
    }
    Ok(())
}

fn write_flow_labels(html: &mut String, labels: &FlowLabelsGeometry) -> Result<(), RenderError> {
    writeln!(html, "  <div class=\"flow-labels\">")?;
    for label in &labels.items {
        writeln!(
            html,
            "    <div class=\"flow-label\"><span class=\"arrow\">\u{2193}</span> {}</div>",
            escape_html(label),
        )?;
    }
    writeln!(html, "  </div>")?;
    Ok(())
}

fn write_container(
    html: &mut String,
    container: &ContainerGeometry<'_>,
    parent_label: Option<&str>,
) -> Result<(), RenderError> {
    let border_class = format!("container-{}", container.border);
    let label_color = container.label_color;

    if let Some(label) = parent_label {
        writeln!(
            html,
            "    <div class=\"tier-label\">{}</div>",
            escape_html(label)
        )?;
    }

    writeln!(html, "    <div class=\"{border_class}\">")?;
    writeln!(
        html,
        "      <div class=\"container-label\" style=\"color: var(--accent-{label_color})\">{}</div>",
        escape_html(&container.label),
    )?;

    for layer in &container.layers {
        match layer {
            LayerGeometry::Tier(tier) => {
                if let Some(label) = &tier.label {
                    writeln!(
                        html,
                        "      <div class=\"tier-label\">{}</div>",
                        escape_html(label)
                    )?;
                }
                if let Some(nested_container) = &tier.container {
                    write_container(html, nested_container, None)?;
                } else {
                    write_node_grid(html, &tier.nodes, tier.columns, false)?;
                }
            }
            LayerGeometry::Connector(conn) => write_connector(html, conn)?,
            LayerGeometry::FlowLabels(labels) => write_flow_labels(html, labels)?,
        }
    }

    writeln!(html, "    </div>")?;
    Ok(())
}

fn write_legend(html: &mut String, plan: &LayoutPlan<'_>) -> Result<(), RenderError> {
    if plan.legend.entries.is_empty() {
        return Ok(());
    }

    writeln!(html, "  <div class=\"legend\">")?;
    for entry in &plan.legend.entries {
        let color = entry.color();
        writeln!(
            html,
            "    <div class=\"legend-item\"><div class=\"swatch {color}\"></div> {}</div>",
            escape_html(entry.label()),
        )?;
    }
    writeln!(html, "  </div>")?;
    Ok(())
}
