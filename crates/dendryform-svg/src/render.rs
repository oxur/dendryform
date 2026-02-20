//! SVG rendering — transforms a LayoutPlan + Theme into a self-contained SVG string.

use std::fmt::Write;

use dendryform_core::{ContainerBorder, Theme};
use dendryform_layout::LayoutPlan;

use crate::defs::write_defs;
use crate::error::SvgError;
use crate::escape::escape_xml;
use crate::metrics::SvgMetrics;
use crate::resolve::{
    ResolvedConnector, ResolvedContainer, ResolvedLayer, ResolvedLegendEntry, ResolvedNode,
    ResolvedPlan, ResolvedTier, resolve,
};

/// Renders a layout plan and theme into a self-contained SVG string.
///
/// The `width` parameter controls the SVG viewport width in pixels.
pub fn render_svg(plan: &LayoutPlan<'_>, theme: &Theme, width: f32) -> Result<String, SvgError> {
    let resolved = resolve(plan, theme, width);
    let mut svg = String::with_capacity(16384);

    write_svg_open(&mut svg, &resolved, theme)?;
    write_defs(&mut svg, theme)?;
    write_header(&mut svg, &resolved, theme)?;

    for layer in &resolved.layers {
        write_layer(&mut svg, layer, theme, &resolved)?;
    }

    write_legend(&mut svg, &resolved.legend_entries, theme)?;

    writeln!(svg, "</svg>")?;
    Ok(svg)
}

fn write_svg_open(
    svg: &mut String,
    plan: &ResolvedPlan<'_>,
    theme: &Theme,
) -> Result<(), SvgError> {
    writeln!(
        svg,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" width=\"{}\" height=\"{}\">",
        plan.width, plan.height, plan.width, plan.height
    )?;
    // Background
    writeln!(
        svg,
        "  <rect width=\"100%\" height=\"100%\" fill=\"{}\"/>",
        theme.backgrounds().page()
    )?;
    Ok(())
}

fn write_header(svg: &mut String, plan: &ResolvedPlan<'_>, theme: &Theme) -> Result<(), SvgError> {
    let center_x = plan.width / 2.0;
    let display_font = theme.fonts().display();

    // Title with accent span
    let accent_color = theme
        .palette()
        .get(dendryform_core::Color::Green)
        .map(|cs| cs.accent())
        .unwrap_or("#3ddc84");

    writeln!(
        svg,
        "  <text x=\"{center_x}\" y=\"{}\" text-anchor=\"middle\" font-family=\"'{display_font}', monospace\" font-weight=\"600\" font-size=\"28\" fill=\"{}\">",
        plan.title_y,
        theme.text().bright()
    )?;
    writeln!(
        svg,
        "    <tspan fill=\"{accent_color}\">{}</tspan> \u{00b7} {}",
        escape_xml(&plan.title_accent),
        escape_xml(&plan.title_text)
    )?;
    writeln!(svg, "  </text>")?;

    // Subtitle
    writeln!(
        svg,
        "  <text x=\"{center_x}\" y=\"{}\" text-anchor=\"middle\" font-family=\"'{display_font}', monospace\" font-weight=\"300\" font-size=\"13\" fill=\"{}\">{}",
        plan.subtitle_y,
        theme.text().dim(),
        escape_xml(&plan.subtitle)
    )?;
    writeln!(svg, "  </text>")?;
    Ok(())
}

fn write_layer(
    svg: &mut String,
    layer: &ResolvedLayer<'_>,
    theme: &Theme,
    plan: &ResolvedPlan<'_>,
) -> Result<(), SvgError> {
    match layer {
        ResolvedLayer::Tier(tier) => write_tier(svg, tier, theme),
        ResolvedLayer::Connector(conn) => write_connector(svg, conn, theme),
        ResolvedLayer::FlowLabels(labels) => write_flow_labels(svg, labels, theme, plan),
    }
}

fn write_tier(svg: &mut String, tier: &ResolvedTier<'_>, theme: &Theme) -> Result<(), SvgError> {
    let display_font = theme.fonts().display();

    // Tier label
    if let Some(label) = &tier.label {
        let label_upper = label.to_uppercase();
        writeln!(
            svg,
            "  <text x=\"{}\" y=\"{}\" font-family=\"'{display_font}', monospace\" font-weight=\"500\" font-size=\"10\" letter-spacing=\"2\" fill=\"{}\">{}</text>",
            tier.rect.x + 4.0,
            tier.label_y,
            theme.text().dim(),
            escape_xml(&label_upper)
        )?;
    }

    // Container
    if let Some(container) = &tier.container {
        write_container(svg, container, theme)?;
        return Ok(());
    }

    // Nodes
    for node in &tier.nodes {
        write_node(svg, node, theme, tier.is_single)?;
    }

    Ok(())
}

fn write_node(
    svg: &mut String,
    node: &ResolvedNode<'_>,
    theme: &Theme,
    is_single: bool,
) -> Result<(), SvgError> {
    let m = SvgMetrics::default();
    let r = &node.rect;
    let color = node.color;
    let accent = theme
        .palette()
        .get(color)
        .map(|cs| cs.accent())
        .unwrap_or("#ffffff");

    let display_font = theme.fonts().display();
    let body_font = theme.fonts().body();

    // Card group
    writeln!(svg, "  <g>")?;

    // Card background rect
    writeln!(
        svg,
        "    <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>",
        r.x,
        r.y,
        r.w,
        r.h,
        m.node_border_radius,
        theme.backgrounds().card(),
        theme.borders().normal()
    )?;

    // Accent bar at top
    writeln!(
        svg,
        "    <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\" fill=\"{accent}\"/>",
        r.x, r.y, r.w, m.node_accent_bar_height, m.node_border_radius
    )?;

    // Content positioning
    let content_x = r.x + m.node_padding_x;
    let mut cy = r.y + m.node_accent_bar_height + m.node_padding_y;

    // Icon + Title
    let title_anchor = if is_single { "middle" } else { "start" };
    let title_x = if is_single {
        r.x + r.w / 2.0
    } else {
        content_x
    };

    cy += m.node_title_font_size;
    writeln!(
        svg,
        "    <text x=\"{title_x}\" y=\"{cy}\" text-anchor=\"{title_anchor}\" font-family=\"'{display_font}', monospace\" font-weight=\"500\" font-size=\"14\" fill=\"{}\">",
        theme.text().bright()
    )?;
    writeln!(
        svg,
        "      <tspan fill=\"{accent}\">{}</tspan> {}",
        escape_xml(node.node.icon()),
        escape_xml(node.node.title())
    )?;
    writeln!(svg, "    </text>")?;

    // Description
    cy += m.node_title_desc_gap;
    cy += m.node_desc_font_size;

    let desc_anchor = if is_single { "middle" } else { "start" };
    let desc_x = if is_single {
        r.x + r.w / 2.0
    } else {
        content_x
    };

    let available_width = r.w - 2.0 * m.node_padding_x;
    let desc_lines = m.wrap_text(node.node.description(), available_width, m.node_desc_font_size);
    let line_h = m.line_height(m.node_desc_font_size);

    writeln!(
        svg,
        "    <text x=\"{desc_x}\" y=\"{cy}\" text-anchor=\"{desc_anchor}\" font-family=\"'{body_font}', sans-serif\" font-size=\"12\" fill=\"{}\">",
        theme.text().dim()
    )?;
    for (i, line) in desc_lines.iter().enumerate() {
        let dy: f32 = if i == 0 { 0.0 } else { line_h };
        writeln!(
            svg,
            "      <tspan x=\"{desc_x}\" dy=\"{dy}\">{}</tspan>",
            escape_xml(line)
        )?;
    }
    writeln!(svg, "    </text>")?;

    cy += (desc_lines.len() - 1) as f32 * line_h;

    // Tech badges
    let tech = node.node.tech();
    if !tech.is_empty() {
        cy += m.node_tech_margin_top;
        let mut bx = if is_single {
            // Center badges
            let total: f32 = tech
                .iter()
                .map(|t| {
                    m.mono_text_width(&t.to_string(), m.node_tech_font_size)
                        + 2.0 * m.node_tech_pad_x
                })
                .sum::<f32>()
                + (tech.len() as f32 - 1.0) * m.node_tech_gap;
            r.x + (r.w - total) / 2.0
        } else {
            content_x
        };

        for t in tech {
            let text = t.to_string();
            let badge_w = m.mono_text_width(&text, m.node_tech_font_size) + 2.0 * m.node_tech_pad_x;
            let badge_h = m.node_tech_font_size + 2.0 * m.node_tech_pad_y;

            writeln!(
                svg,
                "    <rect x=\"{bx}\" y=\"{cy}\" width=\"{badge_w}\" height=\"{badge_h}\" rx=\"4\" fill=\"rgba(255,255,255,0.04)\" stroke=\"rgba(255,255,255,0.06)\" stroke-width=\"1\"/>"
            )?;
            writeln!(
                svg,
                "    <text x=\"{}\" y=\"{}\" font-family=\"'{display_font}', monospace\" font-size=\"10\" fill=\"{}\">{}",
                bx + m.node_tech_pad_x,
                cy + m.node_tech_pad_y + m.node_tech_font_size,
                theme.text().dim(),
                escape_xml(&text)
            )?;
            writeln!(svg, "    </text>")?;

            bx += badge_w + m.node_tech_gap;
        }
    }

    writeln!(svg, "  </g>")?;
    Ok(())
}

fn write_connector(
    svg: &mut String,
    conn: &ResolvedConnector,
    theme: &Theme,
) -> Result<(), SvgError> {
    let m = SvgMetrics::default();
    let display_font = theme.fonts().display();

    if conn.is_internal {
        // Dots
        let total_width =
            m.dot_count as f32 * m.dot_diameter + (m.dot_count as f32 - 1.0) * m.dot_gap;
        let start_x = conn.center_x - total_width / 2.0;
        let cy = conn.top_y + m.dot_diameter / 2.0;

        for i in 0..m.dot_count {
            let dx = start_x + i as f32 * (m.dot_diameter + m.dot_gap) + m.dot_diameter / 2.0;
            writeln!(
                svg,
                "  <circle cx=\"{dx}\" cy=\"{cy}\" r=\"{}\" fill=\"{}\"/>",
                m.dot_diameter / 2.0,
                theme.borders().highlight()
            )?;
        }
    } else {
        // Vertical gradient line
        let x = conn.center_x;
        let y1 = conn.top_y;
        let y2 = conn.top_y + conn.height;

        writeln!(
            svg,
            "  <line x1=\"{x}\" y1=\"{y1}\" x2=\"{x}\" y2=\"{y2}\" stroke=\"url(#connector-grad)\" stroke-width=\"{}\" marker-end=\"url(#arrowhead)\"/>",
            m.connector_width
        )?;

        // Protocol label
        if let Some(label) = &conn.label {
            let label_x = conn.center_x - m.protocol_label_gap;
            let label_y = conn.top_y + conn.height / 2.0 + m.protocol_label_font_size / 3.0;
            writeln!(
                svg,
                "  <text x=\"{label_x}\" y=\"{label_y}\" text-anchor=\"end\" font-family=\"'{display_font}', monospace\" font-size=\"10\" fill=\"{}\">{}",
                theme.text().dim(),
                escape_xml(label)
            )?;
            writeln!(svg, "  </text>")?;
        }
    }

    Ok(())
}

fn write_container(
    svg: &mut String,
    container: &ResolvedContainer<'_>,
    theme: &Theme,
) -> Result<(), SvgError> {
    let m = SvgMetrics::default();
    let display_font = theme.fonts().display();
    let r = &container.rect;

    // Container rect
    match container.border {
        ContainerBorder::Solid => {
            writeln!(
                svg,
                "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\" fill=\"rgba(17,24,32,0.5)\" stroke=\"{}\" stroke-width=\"1\"/>",
                r.x,
                r.y,
                r.w,
                r.h,
                m.container_border_radius,
                theme.borders().normal()
            )?;
        }
        ContainerBorder::Dashed => {
            writeln!(
                svg,
                "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\" fill=\"none\" stroke=\"rgba(255,255,255,0.1)\" stroke-width=\"1\" stroke-dasharray=\"6 3\"/>",
                r.x, r.y, r.w, r.h, m.node_border_radius
            )?;
        }
        _ => {
            writeln!(
                svg,
                "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\" fill=\"rgba(17,24,32,0.5)\" stroke=\"{}\" stroke-width=\"1\"/>",
                r.x,
                r.y,
                r.w,
                r.h,
                m.container_border_radius,
                theme.borders().normal()
            )?;
        }
    }

    // Container label (floating on top border)
    let accent = theme
        .palette()
        .get(container.label_color)
        .map(|cs| cs.accent())
        .unwrap_or("#ffffff");

    let label_upper = container.label.to_uppercase();
    let label_font_size = match container.border {
        ContainerBorder::Dashed => m.container_dashed_label_font_size,
        _ => m.container_label_font_size,
    };
    let label_letter_spacing = match container.border {
        ContainerBorder::Dashed => 1.5,
        _ => m.container_label_letter_spacing,
    };

    // Background rect behind label to mask the border
    let label_text_width = m.mono_text_width(&label_upper, label_font_size)
        + label_letter_spacing * label_upper.len() as f32;
    let label_bg_padding = 10.0;
    let label_bg_color = match container.border {
        ContainerBorder::Dashed => theme.backgrounds().card(),
        _ => theme.backgrounds().page(),
    };
    writeln!(
        svg,
        "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{label_bg_color}\"/>",
        container.label_x - label_bg_padding / 2.0,
        container.label_y - label_font_size,
        label_text_width + label_bg_padding,
        label_font_size + 4.0
    )?;

    writeln!(
        svg,
        "  <text x=\"{}\" y=\"{}\" font-family=\"'{display_font}', monospace\" font-weight=\"500\" font-size=\"{label_font_size}\" letter-spacing=\"{label_letter_spacing}\" fill=\"{accent}\">{}</text>",
        container.label_x,
        container.label_y,
        escape_xml(&label_upper)
    )?;

    // Nested layers
    for layer in &container.layers {
        match layer {
            ResolvedLayer::Tier(tier) => write_tier(svg, tier, theme)?,
            ResolvedLayer::Connector(conn) => write_connector(svg, conn, theme)?,
            ResolvedLayer::FlowLabels(labels) => {
                // For nested flow labels, create a minimal plan reference
                write_flow_labels_inner(svg, labels, theme, container.rect.x, container.rect.w)?;
            }
        }
    }

    Ok(())
}

fn write_flow_labels(
    svg: &mut String,
    labels: &crate::resolve::ResolvedFlowLabels,
    theme: &Theme,
    plan: &ResolvedPlan<'_>,
) -> Result<(), SvgError> {
    write_flow_labels_inner(
        svg,
        labels,
        theme,
        plan.width / 2.0 - labels.content_width / 2.0,
        labels.content_width,
    )
}

fn write_flow_labels_inner(
    svg: &mut String,
    labels: &crate::resolve::ResolvedFlowLabels,
    theme: &Theme,
    area_x: f32,
    area_width: f32,
) -> Result<(), SvgError> {
    let m = SvgMetrics::default();
    let display_font = theme.fonts().display();

    if labels.items.is_empty() {
        return Ok(());
    }

    // Center all items
    let total_width = labels
        .items
        .iter()
        .map(|item| {
            // Arrow + gap + text
            m.mono_text_width("\u{2193}", m.flow_label_arrow_font_size)
                + 5.0
                + m.mono_text_width(item, m.flow_label_font_size)
        })
        .sum::<f32>()
        + (labels.items.len() as f32 - 1.0) * m.flow_label_gap;

    let center_x = area_x + area_width / 2.0;
    let mut x = center_x - total_width / 2.0;

    for item in &labels.items {
        // Arrow
        write!(
            svg,
            "  <text x=\"{x}\" y=\"{}\" font-family=\"'{display_font}', monospace\" font-size=\"9\" fill=\"{}\">\u{2193}",
            labels.center_y,
            theme.borders().highlight()
        )?;
        writeln!(svg, "</text>")?;

        x += m.mono_text_width("\u{2193}", m.flow_label_arrow_font_size) + 5.0;

        // Label text
        write!(
            svg,
            "  <text x=\"{x}\" y=\"{}\" font-family=\"'{display_font}', monospace\" font-size=\"9\" fill=\"{}\">{}",
            labels.center_y,
            theme.text().dim(),
            escape_xml(item)
        )?;
        writeln!(svg, "</text>")?;

        x += m.mono_text_width(item, m.flow_label_font_size) + m.flow_label_gap;
    }

    Ok(())
}

fn write_legend(
    svg: &mut String,
    entries: &[ResolvedLegendEntry],
    theme: &Theme,
) -> Result<(), SvgError> {
    if entries.is_empty() {
        return Ok(());
    }

    let m = SvgMetrics::default();
    let display_font = theme.fonts().display();

    for entry in entries {
        let accent = theme
            .palette()
            .get(entry.color)
            .map(|cs| cs.accent())
            .unwrap_or("#ffffff");

        // Swatch
        writeln!(
            svg,
            "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\" fill=\"{accent}\"/>",
            entry.x, entry.y, m.legend_swatch_size, m.legend_swatch_size, m.legend_swatch_radius
        )?;

        // Label
        let text_x = entry.x + m.legend_swatch_size + m.legend_swatch_text_gap;
        let text_y = entry.y + m.legend_swatch_size - 1.0; // align baseline
        writeln!(
            svg,
            "  <text x=\"{text_x}\" y=\"{text_y}\" font-family=\"'{display_font}', monospace\" font-size=\"10\" fill=\"{}\">{}",
            theme.text().dim(),
            escape_xml(&entry.label)
        )?;
        writeln!(svg, "  </text>")?;
    }

    Ok(())
}
