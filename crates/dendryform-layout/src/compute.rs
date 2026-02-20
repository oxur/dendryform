//! Layout computation — transforms a Diagram into a LayoutPlan.

use dendryform_core::{Diagram, Layer, Tier, TierLayout};

use crate::error::LayoutError;
use crate::geometry::{
    ConnectorGeometry, ContainerGeometry, FlowLabelsGeometry, HeaderGeometry, LayerGeometry,
    LayoutPlan, LegendGeometry, NodeGeometry, TierGeometry, ViewportHint,
};

/// Computes a layout plan from a validated diagram.
///
/// Walks tiers top-to-bottom, assigns grid positions based on each tier's
/// layout configuration, and recurses into containers for nested layouts.
pub fn compute_layout(diagram: &Diagram) -> Result<LayoutPlan<'_>, LayoutError> {
    let header = HeaderGeometry {
        title_text: diagram.header().title().text().to_owned(),
        title_accent: diagram.header().title().accent().to_owned(),
        subtitle: diagram.header().subtitle().to_owned(),
    };

    let layers = compute_layers(diagram.layers(), false)?;

    let legend = LegendGeometry {
        entries: diagram.legend().to_vec(),
    };

    Ok(LayoutPlan {
        viewport: ViewportHint::default(),
        header,
        layers,
        legend,
    })
}

/// Computes layer geometries for a list of layers.
fn compute_layers<'a>(
    layers: &'a [Layer],
    is_internal: bool,
) -> Result<Vec<LayerGeometry<'a>>, LayoutError> {
    let mut result = Vec::with_capacity(layers.len());

    for layer in layers {
        match layer {
            Layer::Tier(tier) => {
                result.push(LayerGeometry::Tier(compute_tier(tier)?));
            }
            Layer::Connector(conn) => {
                result.push(LayerGeometry::Connector(ConnectorGeometry {
                    style: conn.style(),
                    label: conn.label().map(|s| s.to_owned()),
                    is_internal,
                }));
            }
            Layer::FlowLabels(labels) => {
                result.push(LayerGeometry::FlowLabels(FlowLabelsGeometry {
                    items: labels.items().iter().map(|s| s.to_owned()).collect(),
                }));
            }
            _ => {
                // non_exhaustive: skip unknown layer variants gracefully
            }
        }
    }

    Ok(result)
}

/// Computes the geometry for a single tier.
fn compute_tier(tier: &Tier) -> Result<TierGeometry<'_>, LayoutError> {
    let columns = resolve_columns(tier.layout(), tier.nodes().len());

    let nodes: Vec<NodeGeometry<'_>> = tier
        .nodes()
        .iter()
        .enumerate()
        .map(|(i, node)| NodeGeometry {
            node,
            grid_column: i % columns,
            grid_row: i / columns,
        })
        .collect();

    let container = if let Some(c) = tier.container() {
        let nested_layers = compute_layers(c.layers(), true)?;
        Some(ContainerGeometry {
            label: c.label().to_owned(),
            border: c.border(),
            label_color: c.label_color(),
            layers: nested_layers,
        })
    } else {
        None
    };

    Ok(TierGeometry {
        id: tier.id().clone(),
        label: tier.label().map(|s| s.to_owned()),
        layout: tier.layout().clone(),
        columns,
        nodes,
        container,
    })
}

/// Resolves the number of grid columns for a tier layout.
fn resolve_columns(layout: &TierLayout, node_count: usize) -> usize {
    match layout {
        TierLayout::Single => 1,
        TierLayout::Grid { columns } => *columns as usize,
        TierLayout::Auto => node_count.clamp(1, 4),
        _ => node_count.clamp(1, 4), // non_exhaustive fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dendryform_core::{
        Color, Connector, ConnectorStyle, Container, ContainerBorder, Diagram, DiagramHeader, Edge,
        FlowLabels, Layer, LegendEntry, Node, NodeId, NodeKind, RawDiagram, Tier, TierLayout,
    };

    fn test_node(id: &str) -> Node {
        Node::builder()
            .id(NodeId::new(id).unwrap())
            .kind(NodeKind::System)
            .color(Color::Blue)
            .icon("◇")
            .title(id)
            .description("test node")
            .build()
            .unwrap()
    }

    fn make_diagram(layers: Vec<Layer>, edges: Vec<Edge>, legend: Vec<LegendEntry>) -> Diagram {
        let raw = RawDiagram {
            diagram: DiagramHeader::new(Title::new("test", "accent"), "subtitle", "dark"),
            layers,
            legend,
            edges,
        };
        Diagram::try_from(raw).unwrap()
    }

    use dendryform_core::Title;

    #[test]
    fn test_single_tier_layout() {
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![test_node("a"), test_node("b"), test_node("c")],
            ))],
            vec![],
            vec![],
        );

        let plan = compute_layout(&diagram).unwrap();
        assert_eq!(plan.header.title_text, "test");
        assert_eq!(plan.header.title_accent, "accent");
        assert_eq!(plan.header.subtitle, "subtitle");
        assert_eq!(plan.layers.len(), 1);

        if let LayerGeometry::Tier(tier) = &plan.layers[0] {
            assert_eq!(tier.nodes.len(), 3);
            // Auto layout: 3 nodes → 3 columns
            assert_eq!(tier.columns, 3);
            assert_eq!(tier.nodes[0].grid_column, 0);
            assert_eq!(tier.nodes[1].grid_column, 1);
            assert_eq!(tier.nodes[2].grid_column, 2);
            assert_eq!(tier.nodes[0].grid_row, 0);
        } else {
            panic!("expected tier layer");
        }
    }

    #[test]
    fn test_grid_layout_columns() {
        let mut tier = Tier::new(
            NodeId::new("grid").unwrap(),
            vec![
                test_node("a"),
                test_node("b"),
                test_node("c"),
                test_node("d"),
                test_node("e"),
            ],
        );
        tier.set_layout(TierLayout::Grid { columns: 3 });
        let diagram = make_diagram(vec![Layer::Tier(tier)], vec![], vec![]);

        let plan = compute_layout(&diagram).unwrap();
        if let LayerGeometry::Tier(tier) = &plan.layers[0] {
            assert_eq!(tier.columns, 3);
            // 5 nodes in 3 columns: row 0 = [0,1,2], row 1 = [3,4]
            assert_eq!(tier.nodes[0].grid_column, 0);
            assert_eq!(tier.nodes[0].grid_row, 0);
            assert_eq!(tier.nodes[3].grid_column, 0);
            assert_eq!(tier.nodes[3].grid_row, 1);
            assert_eq!(tier.nodes[4].grid_column, 1);
            assert_eq!(tier.nodes[4].grid_row, 1);
        } else {
            panic!("expected tier layer");
        }
    }

    #[test]
    fn test_single_layout_one_column() {
        let mut tier = Tier::new(NodeId::new("single").unwrap(), vec![test_node("a")]);
        tier.set_layout(TierLayout::Single);
        let diagram = make_diagram(vec![Layer::Tier(tier)], vec![], vec![]);

        let plan = compute_layout(&diagram).unwrap();
        if let LayerGeometry::Tier(tier) = &plan.layers[0] {
            assert_eq!(tier.columns, 1);
        } else {
            panic!("expected tier layer");
        }
    }

    #[test]
    fn test_auto_layout_caps_at_four() {
        let tier = Tier::new(
            NodeId::new("many").unwrap(),
            vec![
                test_node("a"),
                test_node("b"),
                test_node("c"),
                test_node("d"),
                test_node("e"),
                test_node("f"),
            ],
        );
        let diagram = make_diagram(vec![Layer::Tier(tier)], vec![], vec![]);

        let plan = compute_layout(&diagram).unwrap();
        if let LayerGeometry::Tier(tier) = &plan.layers[0] {
            assert_eq!(tier.columns, 4);
        } else {
            panic!("expected tier layer");
        }
    }

    #[test]
    fn test_connector_geometry() {
        let diagram = make_diagram(
            vec![
                Layer::Tier(Tier::new(NodeId::new("top").unwrap(), vec![test_node("a")])),
                Layer::Connector(Connector::with_label(ConnectorStyle::Line, "HTTPS")),
                Layer::Tier(Tier::new(
                    NodeId::new("bottom").unwrap(),
                    vec![test_node("b")],
                )),
            ],
            vec![],
            vec![],
        );

        let plan = compute_layout(&diagram).unwrap();
        assert_eq!(plan.layers.len(), 3);

        if let LayerGeometry::Connector(conn) = &plan.layers[1] {
            assert_eq!(conn.style, ConnectorStyle::Line);
            assert_eq!(conn.label.as_deref(), Some("HTTPS"));
            assert!(!conn.is_internal);
        } else {
            panic!("expected connector layer");
        }
    }

    #[test]
    fn test_container_nesting() {
        let container = Container::new(
            "server",
            ContainerBorder::Solid,
            Color::Green,
            vec![Layer::Tier(Tier::new(
                NodeId::new("inner").unwrap(),
                vec![test_node("api")],
            ))],
        );
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::with_container(
                NodeId::new("server").unwrap(),
                container,
            ))],
            vec![],
            vec![],
        );

        let plan = compute_layout(&diagram).unwrap();
        if let LayerGeometry::Tier(tier) = &plan.layers[0] {
            assert!(tier.container.is_some());
            let c = tier.container.as_ref().unwrap();
            assert_eq!(c.label, "server");
            assert_eq!(c.border, ContainerBorder::Solid);
            assert_eq!(c.label_color, Color::Green);
            assert_eq!(c.layers.len(), 1);

            if let LayerGeometry::Tier(inner) = &c.layers[0] {
                assert_eq!(inner.nodes.len(), 1);
                assert_eq!(inner.nodes[0].node.id().as_str(), "api");
            } else {
                panic!("expected inner tier");
            }
        } else {
            panic!("expected tier layer");
        }
    }

    #[test]
    fn test_internal_connector_flag() {
        let container = Container::new(
            "server",
            ContainerBorder::Solid,
            Color::Green,
            vec![
                Layer::Tier(Tier::new(NodeId::new("top").unwrap(), vec![test_node("a")])),
                Layer::Connector(Connector::new(ConnectorStyle::Dots)),
                Layer::Tier(Tier::new(
                    NodeId::new("bottom").unwrap(),
                    vec![test_node("b")],
                )),
            ],
        );
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::with_container(
                NodeId::new("server").unwrap(),
                container,
            ))],
            vec![],
            vec![],
        );

        let plan = compute_layout(&diagram).unwrap();
        if let LayerGeometry::Tier(tier) = &plan.layers[0] {
            let c = tier.container.as_ref().unwrap();
            if let LayerGeometry::Connector(conn) = &c.layers[1] {
                assert!(conn.is_internal, "container connectors should be internal");
                assert_eq!(conn.style, ConnectorStyle::Dots);
            } else {
                panic!("expected connector");
            }
        } else {
            panic!("expected tier");
        }
    }

    #[test]
    fn test_flow_labels_geometry() {
        let diagram = make_diagram(
            vec![
                Layer::Tier(Tier::new(NodeId::new("top").unwrap(), vec![test_node("a")])),
                Layer::FlowLabels(FlowLabels::new(vec![
                    "SQL queries".to_owned(),
                    "cache reads".to_owned(),
                ])),
                Layer::Tier(Tier::new(
                    NodeId::new("bottom").unwrap(),
                    vec![test_node("b")],
                )),
            ],
            vec![],
            vec![],
        );

        let plan = compute_layout(&diagram).unwrap();
        if let LayerGeometry::FlowLabels(fl) = &plan.layers[1] {
            assert_eq!(fl.items.len(), 2);
            assert_eq!(fl.items[0], "SQL queries");
        } else {
            panic!("expected flow labels");
        }
    }

    #[test]
    fn test_legend_geometry() {
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![test_node("a")],
            ))],
            vec![],
            vec![
                LegendEntry::new(Color::Blue, "Clients"),
                LegendEntry::new(Color::Green, "Servers"),
            ],
        );

        let plan = compute_layout(&diagram).unwrap();
        assert_eq!(plan.legend.entries.len(), 2);
        assert_eq!(plan.legend.entries[0].label(), "Clients");
        assert_eq!(plan.legend.entries[1].color(), Color::Green);
    }

    #[test]
    fn test_viewport_defaults() {
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![test_node("a")],
            ))],
            vec![],
            vec![],
        );

        let plan = compute_layout(&diagram).unwrap();
        assert_eq!(plan.viewport.width, 1100.0);
        assert_eq!(plan.viewport.padding_x, 32.0);
    }

    #[test]
    fn test_taproot_layout() {
        let yaml = include_str!("../../../examples/taproot/architecture.yaml");
        let diagram: Diagram = dendryform_parse::parse_yaml(yaml).unwrap();
        let plan = compute_layout(&diagram).unwrap();

        assert_eq!(plan.header.title_accent, "taproot");
        assert_eq!(plan.layers.len(), 5);
        assert_eq!(plan.legend.entries.len(), 6);

        // First layer: client tier with 1 node, single layout
        if let LayerGeometry::Tier(tier) = &plan.layers[0] {
            assert_eq!(tier.nodes.len(), 1);
            assert_eq!(tier.columns, 1);
        } else {
            panic!("expected client tier");
        }

        // Second layer: connector
        assert!(matches!(plan.layers[1], LayerGeometry::Connector(_)));

        // Third layer: server tier with container
        if let LayerGeometry::Tier(tier) = &plan.layers[2] {
            assert!(tier.container.is_some());
            let c = tier.container.as_ref().unwrap();
            assert_eq!(c.label, "taproot server · cloud run");
            // Container has: edge tier, dots, tools tier, dots, intelligence tier, infrastructure tier
            assert!(c.layers.len() >= 4);
        } else {
            panic!("expected server tier");
        }

        // Fourth layer: flow labels
        if let LayerGeometry::FlowLabels(fl) = &plan.layers[3] {
            assert_eq!(fl.items.len(), 3);
        } else {
            panic!("expected flow labels");
        }

        // Fifth layer: external services tier
        if let LayerGeometry::Tier(tier) = &plan.layers[4] {
            assert_eq!(tier.nodes.len(), 3);
            assert_eq!(tier.columns, 3);
        } else {
            panic!("expected external services tier");
        }
    }

    #[test]
    fn test_ai_kasu_layout() {
        let yaml = include_str!("../../../examples/ai-kasu/architecture.yaml");
        let diagram: Diagram = dendryform_parse::parse_yaml(yaml).unwrap();
        let plan = compute_layout(&diagram).unwrap();

        assert_eq!(plan.header.title_accent, "ai-kasu");
        assert_eq!(plan.layers.len(), 7);
        assert_eq!(plan.legend.entries.len(), 6);

        // First layer: MCP Clients tier with 3 nodes in 3-column grid
        if let LayerGeometry::Tier(tier) = &plan.layers[0] {
            assert_eq!(tier.nodes.len(), 3);
            assert_eq!(tier.columns, 3);
        } else {
            panic!("expected clients tier");
        }

        // Second layer: connector
        assert!(matches!(plan.layers[1], LayerGeometry::Connector(_)));

        // Third layer: server tier with nested container (solid + dashed)
        if let LayerGeometry::Tier(tier) = &plan.layers[2] {
            assert!(tier.container.is_some());
            let c = tier.container.as_ref().unwrap();
            assert_eq!(c.label, "kasu-server · rust binary");
        } else {
            panic!("expected server tier");
        }
    }

    #[test]
    fn test_oxur_lisp_layout() {
        let yaml = include_str!("../../../examples/oxur-lisp/architecture.yaml");
        let diagram: Diagram = dendryform_parse::parse_yaml(yaml).unwrap();
        let plan = compute_layout(&diagram).unwrap();

        assert_eq!(plan.header.title_accent, "oxur");
        assert_eq!(plan.layers.len(), 9);
        assert_eq!(plan.legend.entries.len(), 6);

        // First layer: source input (single node)
        if let LayerGeometry::Tier(tier) = &plan.layers[0] {
            assert_eq!(tier.nodes.len(), 1);
            assert_eq!(tier.columns, 1);
        } else {
            panic!("expected source input tier");
        }

        // Second layer: connector
        assert!(matches!(plan.layers[1], LayerGeometry::Connector(_)));

        // Third layer: pipeline container
        if let LayerGeometry::Tier(tier) = &plan.layers[2] {
            assert!(tier.container.is_some());
            let c = tier.container.as_ref().unwrap();
            assert_eq!(c.label, "oxur compilation pipeline");
        } else {
            panic!("expected pipeline tier");
        }
    }
}
