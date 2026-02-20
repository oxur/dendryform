//! Grid-to-pixel coordinate resolver.
//!
//! Converts the relative grid indices in a `LayoutPlan` into absolute pixel
//! rectangles for SVG rendering.

use dendryform_core::{Color, ConnectorStyle, ContainerBorder, LegendEntry, Node, Theme};
use dendryform_layout::{
    ConnectorGeometry, ContainerGeometry, FlowLabelsGeometry, LayerGeometry, LayoutPlan,
    TierGeometry,
};

use crate::metrics::SvgMetrics;

/// An absolute pixel rectangle.
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

/// A resolved node with absolute position and color.
#[derive(Debug)]
pub struct ResolvedNode<'a> {
    pub node: &'a Node,
    pub rect: Rect,
    pub color: Color,
}

/// A resolved tier with absolute positions.
#[derive(Debug)]
pub struct ResolvedTier<'a> {
    pub rect: Rect,
    pub label: Option<String>,
    pub label_y: f32,
    pub nodes: Vec<ResolvedNode<'a>>,
    pub container: Option<ResolvedContainer<'a>>,
    pub is_single: bool,
}

/// A resolved connector.
#[derive(Debug)]
#[allow(dead_code)]
pub struct ResolvedConnector {
    pub center_x: f32,
    pub top_y: f32,
    pub height: f32,
    pub style: ConnectorStyle,
    pub label: Option<String>,
    pub is_internal: bool,
}

/// A resolved container with border and nested layers.
#[derive(Debug)]
pub struct ResolvedContainer<'a> {
    pub rect: Rect,
    pub border: ContainerBorder,
    pub label: String,
    pub label_color: Color,
    pub label_x: f32,
    pub label_y: f32,
    pub layers: Vec<ResolvedLayer<'a>>,
}

/// A resolved flow label set.
#[derive(Debug)]
pub struct ResolvedFlowLabels {
    pub center_y: f32,
    pub items: Vec<String>,
    pub content_width: f32,
}

/// A single resolved layer.
#[derive(Debug)]
pub enum ResolvedLayer<'a> {
    Tier(ResolvedTier<'a>),
    Connector(ResolvedConnector),
    FlowLabels(ResolvedFlowLabels),
}

/// A resolved legend entry.
#[derive(Debug)]
pub struct ResolvedLegendEntry {
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub label: String,
}

/// The fully resolved plan with absolute pixel coordinates.
#[derive(Debug)]
pub struct ResolvedPlan<'a> {
    pub width: f32,
    pub height: f32,
    pub title_text: String,
    pub title_accent: String,
    pub title_y: f32,
    pub subtitle: String,
    pub subtitle_y: f32,
    pub layers: Vec<ResolvedLayer<'a>>,
    pub legend_entries: Vec<ResolvedLegendEntry>,
}

/// Resolves a layout plan into absolute pixel coordinates.
pub fn resolve<'a>(plan: &'a LayoutPlan<'a>, theme: &Theme, width: f32) -> ResolvedPlan<'a> {
    let m = SvgMetrics::default();
    let _ = theme; // Theme is used only for palette lookups in rendering

    let content_width = width - 2.0 * m.padding_x;
    let mut y = m.padding_top;

    // Header
    let title_y = y + m.title_font_size;
    y += m.title_font_size + m.title_subtitle_gap;
    let subtitle_y = y + m.subtitle_font_size;
    y += m.subtitle_font_size + m.header_margin_bottom;

    // Layers
    let mut layers = Vec::new();
    for layer in &plan.layers {
        let resolved = resolve_layer(layer, m.padding_x, &mut y, content_width, &m);
        layers.push(resolved);
    }

    // Legend
    let legend_entries =
        resolve_legend(&plan.legend.entries, m.padding_x, &mut y, content_width, &m);

    y += m.padding_bottom;

    ResolvedPlan {
        width,
        height: y,
        title_text: plan.header.title_text.clone(),
        title_accent: plan.header.title_accent.clone(),
        title_y,
        subtitle: plan.header.subtitle.clone(),
        subtitle_y,
        layers,
        legend_entries,
    }
}

fn resolve_layer<'a>(
    layer: &'a LayerGeometry<'a>,
    x_offset: f32,
    y: &mut f32,
    content_width: f32,
    m: &SvgMetrics,
) -> ResolvedLayer<'a> {
    match layer {
        LayerGeometry::Tier(tier) => {
            ResolvedLayer::Tier(resolve_tier(tier, x_offset, y, content_width, m))
        }
        LayerGeometry::Connector(conn) => {
            ResolvedLayer::Connector(resolve_connector(conn, x_offset, y, content_width, m))
        }
        LayerGeometry::FlowLabels(labels) => {
            ResolvedLayer::FlowLabels(resolve_flow_labels(labels, y, content_width, x_offset, m))
        }
    }
}

fn resolve_tier<'a>(
    tier: &'a TierGeometry<'a>,
    x_offset: f32,
    y: &mut f32,
    content_width: f32,
    m: &SvgMetrics,
) -> ResolvedTier<'a> {
    let tier_start_y = *y;

    // Tier label
    let label_y = if tier.label.is_some() {
        let ly = *y + m.tier_label_font_size;
        *y += m.tier_label_font_size + m.tier_label_margin_bottom;
        ly
    } else {
        *y
    };

    if let Some(container) = &tier.container {
        let container_resolved = resolve_container(container, x_offset, y, content_width, m);
        let tier_rect = Rect {
            x: x_offset,
            y: tier_start_y,
            w: content_width,
            h: *y - tier_start_y,
        };
        *y += m.tier_margin_bottom;
        return ResolvedTier {
            rect: tier_rect,
            label: tier.label.clone(),
            label_y,
            nodes: Vec::new(),
            container: Some(container_resolved),
            is_single: false,
        };
    }

    // Resolve nodes in grid
    let columns = tier.columns.max(1);
    let is_single = columns == 1;
    let cell_width = (content_width - (columns as f32 - 1.0) * m.grid_gap) / columns as f32;

    // Group nodes by row
    let max_row = tier.nodes.iter().map(|n| n.grid_row).max().unwrap_or(0);
    let mut resolved_nodes = Vec::new();
    for row in 0..=max_row {
        let row_nodes: Vec<_> = tier.nodes.iter().filter(|n| n.grid_row == row).collect();
        if row_nodes.is_empty() {
            continue;
        }

        // Compute max node height in this row
        let row_height = row_nodes
            .iter()
            .map(|ng| compute_node_height(ng.node, m, cell_width))
            .fold(0.0_f32, f32::max);

        for ng in &row_nodes {
            let col = ng.grid_column;
            let nx = x_offset + col as f32 * (cell_width + m.grid_gap);
            let node_h = compute_node_height(ng.node, m, cell_width);
            let ny = *y;

            resolved_nodes.push(ResolvedNode {
                node: ng.node,
                rect: Rect {
                    x: nx,
                    y: ny,
                    w: cell_width,
                    h: node_h,
                },
                color: ng.node.color(),
            });
        }

        *y += row_height + m.grid_gap;
    }

    // Replace last grid_gap with grid_margin_bottom
    if !resolved_nodes.is_empty() {
        *y -= m.grid_gap;
        *y += m.grid_margin_bottom;
    }

    let tier_rect = Rect {
        x: x_offset,
        y: tier_start_y,
        w: content_width,
        h: *y - tier_start_y,
    };

    *y += m.tier_margin_bottom;

    ResolvedTier {
        rect: tier_rect,
        label: tier.label.clone(),
        label_y,
        nodes: resolved_nodes,
        container: None,
        is_single,
    }
}

fn compute_node_height(node: &Node, m: &SvgMetrics, cell_width: f32) -> f32 {
    let available_width = cell_width - 2.0 * m.node_padding_x;
    let desc_lines = m.wrap_text(node.description(), available_width, m.node_desc_font_size);

    let mut h = m.node_accent_bar_height;
    h += m.node_padding_y; // top padding
    h += m.node_title_font_size; // title line
    h += m.node_title_desc_gap;
    h += desc_lines.len() as f32 * m.line_height(m.node_desc_font_size); // wrapped desc

    if !node.tech().is_empty() {
        h += m.node_tech_margin_top;
        h += m.node_tech_font_size + 2.0 * m.node_tech_pad_y + 2.0; // badge height + border
    }

    h += m.node_padding_y; // bottom padding
    h
}

fn resolve_connector(
    conn: &ConnectorGeometry,
    x_offset: f32,
    y: &mut f32,
    content_width: f32,
    m: &SvgMetrics,
) -> ResolvedConnector {
    let center_x = x_offset + content_width / 2.0;

    if conn.is_internal {
        *y += m.dot_margin_y;
        let top_y = *y;
        let total_dots_width =
            m.dot_count as f32 * m.dot_diameter + (m.dot_count as f32 - 1.0) * m.dot_gap;
        let _ = total_dots_width; // used in rendering, not positioning
        *y += m.dot_diameter + m.dot_margin_y;

        ResolvedConnector {
            center_x,
            top_y,
            height: m.dot_diameter,
            style: conn.style,
            label: conn.label.clone(),
            is_internal: true,
        }
    } else {
        *y += m.connector_margin_y;
        let top_y = *y;
        *y += m.connector_height;
        *y += m.connector_margin_y;

        ResolvedConnector {
            center_x,
            top_y,
            height: m.connector_height,
            style: conn.style,
            label: conn.label.clone(),
            is_internal: false,
        }
    }
}

fn resolve_container<'a>(
    container: &'a ContainerGeometry<'a>,
    x_offset: f32,
    y: &mut f32,
    content_width: f32,
    m: &SvgMetrics,
) -> ResolvedContainer<'a> {
    let padding = match container.border {
        ContainerBorder::Solid => m.container_solid_padding,
        ContainerBorder::Dashed => m.container_dashed_padding,
        _ => m.container_solid_padding,
    };

    let container_x = x_offset;
    let container_start_y = *y;

    // Label sits on the top border
    let label_x = container_x + padding;
    let label_y = *y - m.container_label_y_offset;

    // Inner content area
    let inner_x = container_x + padding;
    let inner_width = content_width - 2.0 * padding;

    // Space for label that overlaps the top
    *y += padding;

    let mut inner_layers = Vec::new();
    for layer in &container.layers {
        let resolved = resolve_layer(layer, inner_x, y, inner_width, m);
        inner_layers.push(resolved);
    }

    *y += padding;

    let container_rect = Rect {
        x: container_x,
        y: container_start_y,
        w: content_width,
        h: *y - container_start_y,
    };

    if matches!(container.border, ContainerBorder::Dashed) {
        *y += m.container_margin_bottom;
    }

    ResolvedContainer {
        rect: container_rect,
        border: container.border,
        label: container.label.clone(),
        label_color: container.label_color,
        label_x,
        label_y,
        layers: inner_layers,
    }
}

fn resolve_flow_labels(
    labels: &FlowLabelsGeometry,
    y: &mut f32,
    content_width: f32,
    _x_offset: f32,
    m: &SvgMetrics,
) -> ResolvedFlowLabels {
    *y += m.flow_label_margin_y;
    let center_y = *y + m.flow_label_font_size;
    *y += m.flow_label_font_size + m.flow_label_margin_y;

    ResolvedFlowLabels {
        center_y,
        items: labels.items.clone(),
        content_width,
    }
}

fn resolve_legend(
    entries: &[LegendEntry],
    x_offset: f32,
    y: &mut f32,
    content_width: f32,
    m: &SvgMetrics,
) -> Vec<ResolvedLegendEntry> {
    if entries.is_empty() {
        return Vec::new();
    }

    *y += m.legend_margin_top;

    // Compute total legend width to center it
    let item_widths: Vec<f32> = entries
        .iter()
        .map(|e| {
            m.legend_swatch_size
                + m.legend_swatch_text_gap
                + m.mono_text_width(e.label(), m.legend_font_size)
        })
        .collect();
    let total_width: f32 =
        item_widths.iter().sum::<f32>() + (entries.len() as f32 - 1.0) * m.legend_item_gap;

    let start_x = x_offset + (content_width - total_width) / 2.0;
    let entry_y = *y;

    let mut resolved = Vec::new();
    let mut cx = start_x;

    for (i, entry) in entries.iter().enumerate() {
        resolved.push(ResolvedLegendEntry {
            x: cx,
            y: entry_y,
            color: entry.color(),
            label: entry.label().to_owned(),
        });
        cx += item_widths[i] + m.legend_item_gap;
    }

    *y += m.legend_swatch_size + 4.0; // swatch height + small padding

    resolved
}

#[cfg(test)]
mod tests {
    use super::*;
    use dendryform_core::{
        Color, Connector, ConnectorStyle, Container, ContainerBorder, Diagram, DiagramHeader,
        Layer, LegendEntry, Node, NodeId, NodeKind, RawDiagram, Tier, Title,
    };
    use dendryform_layout::compute_layout;

    fn test_node(id: &str, color: Color) -> Node {
        Node::builder()
            .id(NodeId::new(id).unwrap())
            .kind(NodeKind::System)
            .color(color)
            .icon("\u{25c7}")
            .title(id)
            .description("test node")
            .build()
            .unwrap()
    }

    fn make_diagram(layers: Vec<Layer>, legend: Vec<LegendEntry>) -> Diagram {
        let raw = RawDiagram {
            diagram: DiagramHeader::new(Title::new("test", "accent"), "subtitle", "dark"),
            layers,
            legend,
            edges: vec![],
        };
        Diagram::try_from(raw).unwrap()
    }

    #[test]
    fn test_resolve_minimal() {
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![test_node("app", Color::Blue)],
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let theme = Theme::dark();
        let resolved = resolve(&plan, &theme, 1100.0);

        assert!((resolved.width - 1100.0).abs() < f32::EPSILON);
        assert!(resolved.height > 0.0);
        assert_eq!(resolved.layers.len(), 1);
    }

    #[test]
    fn test_resolve_node_positions() {
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![test_node("a", Color::Blue), test_node("b", Color::Green)],
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let theme = Theme::dark();
        let resolved = resolve(&plan, &theme, 1100.0);

        if let ResolvedLayer::Tier(tier) = &resolved.layers[0] {
            assert_eq!(tier.nodes.len(), 2);
            // First node should be at left
            assert!(tier.nodes[0].rect.x < tier.nodes[1].rect.x);
        } else {
            panic!("Expected tier layer");
        }
    }

    #[test]
    fn test_resolve_connector() {
        let diagram = make_diagram(
            vec![
                Layer::Tier(Tier::new(
                    NodeId::new("top").unwrap(),
                    vec![test_node("a", Color::Blue)],
                )),
                Layer::Connector(Connector::with_label(ConnectorStyle::Line, "HTTPS")),
                Layer::Tier(Tier::new(
                    NodeId::new("bottom").unwrap(),
                    vec![test_node("b", Color::Green)],
                )),
            ],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let theme = Theme::dark();
        let resolved = resolve(&plan, &theme, 1100.0);

        assert_eq!(resolved.layers.len(), 3);
        if let ResolvedLayer::Connector(conn) = &resolved.layers[1] {
            assert_eq!(conn.label.as_deref(), Some("HTTPS"));
            assert!(!conn.is_internal);
        } else {
            panic!("Expected connector layer");
        }
    }

    #[test]
    fn test_resolve_legend() {
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![test_node("a", Color::Blue)],
            ))],
            vec![
                LegendEntry::new(Color::Blue, "Clients"),
                LegendEntry::new(Color::Green, "Servers"),
            ],
        );
        let plan = compute_layout(&diagram).unwrap();
        let theme = Theme::dark();
        let resolved = resolve(&plan, &theme, 1100.0);

        assert_eq!(resolved.legend_entries.len(), 2);
        assert_eq!(resolved.legend_entries[0].label, "Clients");
        assert_eq!(resolved.legend_entries[1].label, "Servers");
    }

    #[test]
    fn test_resolve_container() {
        let container = Container::new(
            "server",
            ContainerBorder::Solid,
            Color::Green,
            vec![Layer::Tier(Tier::new(
                NodeId::new("inner").unwrap(),
                vec![test_node("api", Color::Green)],
            ))],
        );
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::with_container(
                NodeId::new("server").unwrap(),
                container,
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let theme = Theme::dark();
        let resolved = resolve(&plan, &theme, 1100.0);

        if let ResolvedLayer::Tier(tier) = &resolved.layers[0] {
            assert!(tier.container.is_some());
            let c = tier.container.as_ref().unwrap();
            assert_eq!(c.label, "server");
            assert!(!c.layers.is_empty());
        } else {
            panic!("Expected tier layer");
        }
    }

    #[test]
    fn test_resolve_dashed_container() {
        let container = Container::new(
            "services",
            ContainerBorder::Dashed,
            Color::Purple,
            vec![Layer::Tier(Tier::new(
                NodeId::new("inner").unwrap(),
                vec![test_node("svc", Color::Purple)],
            ))],
        );
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::with_container(
                NodeId::new("svc-tier").unwrap(),
                container,
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let theme = Theme::dark();
        let resolved = resolve(&plan, &theme, 1100.0);

        if let ResolvedLayer::Tier(tier) = &resolved.layers[0] {
            let c = tier.container.as_ref().unwrap();
            assert_eq!(c.border, ContainerBorder::Dashed);
        } else {
            panic!("Expected tier layer");
        }
    }

    #[test]
    fn test_resolve_internal_connector() {
        let container = Container::new(
            "server",
            ContainerBorder::Solid,
            Color::Green,
            vec![
                Layer::Tier(Tier::new(
                    NodeId::new("top").unwrap(),
                    vec![test_node("a", Color::Green)],
                )),
                Layer::Connector(Connector::new(ConnectorStyle::Dots)),
                Layer::Tier(Tier::new(
                    NodeId::new("bottom").unwrap(),
                    vec![test_node("b", Color::Green)],
                )),
            ],
        );
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::with_container(
                NodeId::new("server").unwrap(),
                container,
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let theme = Theme::dark();
        let resolved = resolve(&plan, &theme, 1100.0);

        if let ResolvedLayer::Tier(tier) = &resolved.layers[0] {
            let c = tier.container.as_ref().unwrap();
            assert!(c.layers.len() >= 3);
            if let ResolvedLayer::Connector(conn) = &c.layers[1] {
                assert!(conn.is_internal);
            } else {
                panic!("Expected connector layer");
            }
        } else {
            panic!("Expected tier layer");
        }
    }

    #[test]
    fn test_resolve_flow_labels() {
        use dendryform_core::FlowLabels;

        let diagram = make_diagram(
            vec![
                Layer::Tier(Tier::new(
                    NodeId::new("top").unwrap(),
                    vec![test_node("a", Color::Blue)],
                )),
                Layer::FlowLabels(FlowLabels::new(vec![
                    "SQL queries".to_owned(),
                    "cache reads".to_owned(),
                ])),
                Layer::Tier(Tier::new(
                    NodeId::new("bottom").unwrap(),
                    vec![test_node("b", Color::Green)],
                )),
            ],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let theme = Theme::dark();
        let resolved = resolve(&plan, &theme, 1100.0);

        if let ResolvedLayer::FlowLabels(fl) = &resolved.layers[1] {
            assert_eq!(fl.items.len(), 2);
            assert!(fl.center_y > 0.0);
        } else {
            panic!("Expected flow labels layer");
        }
    }

    #[test]
    fn test_resolve_tier_with_label() {
        let mut tier = Tier::new(
            NodeId::new("main").unwrap(),
            vec![test_node("a", Color::Blue)],
        );
        tier.set_label("My Tier");
        let diagram = make_diagram(vec![Layer::Tier(tier)], vec![]);
        let plan = compute_layout(&diagram).unwrap();
        let theme = Theme::dark();
        let resolved = resolve(&plan, &theme, 1100.0);

        if let ResolvedLayer::Tier(tier) = &resolved.layers[0] {
            assert_eq!(tier.label.as_deref(), Some("My Tier"));
        } else {
            panic!("Expected tier layer");
        }
    }

    #[test]
    fn test_resolve_empty_legend() {
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![test_node("a", Color::Blue)],
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let theme = Theme::dark();
        let resolved = resolve(&plan, &theme, 1100.0);

        assert!(resolved.legend_entries.is_empty());
    }

    #[test]
    fn test_resolve_multi_column_grid() {
        use dendryform_core::TierLayout;

        let mut tier = Tier::new(
            NodeId::new("grid").unwrap(),
            vec![
                test_node("a", Color::Blue),
                test_node("b", Color::Blue),
                test_node("c", Color::Blue),
            ],
        );
        tier.set_layout(TierLayout::Grid { columns: 2 });
        let diagram = make_diagram(vec![Layer::Tier(tier)], vec![]);
        let plan = compute_layout(&diagram).unwrap();
        let theme = Theme::dark();
        let resolved = resolve(&plan, &theme, 1100.0);

        if let ResolvedLayer::Tier(tier) = &resolved.layers[0] {
            assert_eq!(tier.nodes.len(), 3);
            // First two should be on row 0, third on row 1
            assert!(tier.nodes[0].rect.y < tier.nodes[2].rect.y);
        } else {
            panic!("Expected tier layer");
        }
    }
}
