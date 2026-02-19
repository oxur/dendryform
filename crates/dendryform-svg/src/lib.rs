//! # dendryform-svg
//!
//! Static SVG renderer for dendryform diagrams.
//!
//! Consumes a [`LayoutPlan`](dendryform_layout::LayoutPlan) and
//! [`Theme`](dendryform_core::Theme) to produce a self-contained SVG string
//! with absolute pixel coordinates, embedded font imports, and the dark
//! Taproot theme.
//!
//! ## Quick Start
//!
//! ```no_run
//! use dendryform_core::Theme;
//! use dendryform_svg::render_svg;
//! use dendryform_layout::compute_layout;
//!
//! let diagram = dendryform_parse::parse_yaml_file("examples/taproot/taproot.yml").unwrap();
//! let plan = compute_layout(&diagram).unwrap();
//! let svg = render_svg(&plan, &Theme::dark(), 1100.0).unwrap();
//! std::fs::write("output.svg", svg).unwrap();
//! ```

mod defs;
mod error;
mod escape;
mod metrics;
mod render;
mod resolve;

pub use error::SvgError;
pub use render::render_svg;

/// Returns the version of the dendryform-svg crate.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use dendryform_core::{
        Color, Connector, ConnectorStyle, Container, ContainerBorder, Diagram, DiagramHeader,
        FlowLabels, Layer, LegendEntry, Node, NodeId, NodeKind, RawDiagram, Theme, Tier, Title,
    };
    use dendryform_layout::compute_layout;

    use super::*;

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
    fn test_version_is_set() {
        assert_eq!(version(), "0.0.1");
    }

    #[test]
    fn test_render_minimal() {
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![test_node("app", Color::Blue)],
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let svg = render_svg(&plan, &Theme::dark(), 1100.0).unwrap();

        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("accent"));
        assert!(svg.contains("test"));
    }

    #[test]
    fn test_render_contains_defs() {
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![test_node("app", Color::Blue)],
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let svg = render_svg(&plan, &Theme::dark(), 1100.0).unwrap();

        assert!(svg.contains("<defs>"));
        assert!(svg.contains("arrowhead"));
        assert!(svg.contains("connector-grad"));
        assert!(svg.contains("@import url"));
    }

    #[test]
    fn test_render_contains_node() {
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![test_node("myapp", Color::Green)],
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let svg = render_svg(&plan, &Theme::dark(), 1100.0).unwrap();

        assert!(svg.contains("myapp"));
        // Accent color for green
        assert!(svg.contains("#3ddc84"));
    }

    #[test]
    fn test_render_contains_connector() {
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
        let svg = render_svg(&plan, &Theme::dark(), 1100.0).unwrap();

        assert!(svg.contains("HTTPS"));
        assert!(svg.contains("marker-end"));
    }

    #[test]
    fn test_render_contains_container() {
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
        let svg = render_svg(&plan, &Theme::dark(), 1100.0).unwrap();

        assert!(svg.contains("SERVER"));
        assert!(svg.contains("api"));
    }

    #[test]
    fn test_render_contains_legend() {
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
        let svg = render_svg(&plan, &Theme::dark(), 1100.0).unwrap();

        assert!(svg.contains("Clients"));
        assert!(svg.contains("Servers"));
        // Check for swatch rects (legend_swatch_size = 10)
        assert!(svg.contains("width=\"10\""));
    }

    #[test]
    fn test_render_contains_flow_labels() {
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
                    vec![test_node("b", Color::Red)],
                )),
            ],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let svg = render_svg(&plan, &Theme::dark(), 1100.0).unwrap();

        assert!(svg.contains("SQL queries"));
        assert!(svg.contains("\u{2193}")); // down arrow
    }

    #[test]
    fn test_render_internal_connector() {
        let container = Container::new(
            "server",
            ContainerBorder::Solid,
            Color::Green,
            vec![
                Layer::Tier(Tier::new(
                    NodeId::new("inner1").unwrap(),
                    vec![test_node("a", Color::Green)],
                )),
                Layer::Connector(Connector::new(ConnectorStyle::Dots)),
                Layer::Tier(Tier::new(
                    NodeId::new("inner2").unwrap(),
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
        let svg = render_svg(&plan, &Theme::dark(), 1100.0).unwrap();

        // Should have dot circles
        assert!(svg.contains("<circle"));
    }

    #[test]
    fn test_xml_escaping() {
        let node = Node::builder()
            .id(NodeId::new("test").unwrap())
            .kind(NodeKind::System)
            .color(Color::Blue)
            .icon("<>")
            .title("A & B")
            .description("x < y > z")
            .build()
            .unwrap();
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![node],
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let svg = render_svg(&plan, &Theme::dark(), 1100.0).unwrap();

        assert!(svg.contains("&amp;"));
        assert!(svg.contains("&lt;"));
        assert!(svg.contains("&gt;"));
        assert!(!svg.contains("A & B"));
    }

    #[test]
    fn test_render_background_rect() {
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![test_node("a", Color::Blue)],
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let svg = render_svg(&plan, &Theme::dark(), 1100.0).unwrap();

        assert!(svg.contains("#0a0e14")); // dark theme page background
        assert!(svg.contains("viewBox=\"0 0 1100"));
    }

    #[test]
    fn test_render_taproot_svg() {
        let yaml = include_str!("../../../examples/taproot/taproot.yml");
        let diagram = dendryform_parse::parse_yaml(yaml).unwrap();
        let plan = compute_layout(&diagram).unwrap();
        let svg = render_svg(&plan, &Theme::dark(), 1100.0).unwrap();

        assert!(svg.contains("<svg"));
        assert!(svg.contains("taproot"));
        assert!(svg.contains("system architecture"));
        assert!(svg.contains("Claude Desktop"));
        assert!(svg.contains("Streamable HTTP + SSE"));
        assert!(svg.contains("BigQuery"));
        assert!(svg.contains("SQL queries"));
        assert!(svg.contains("Client / Auth"));
    }

    #[test]
    fn test_render_ai_kasu_svg() {
        let yaml = include_str!("../../../examples/ai-kasu/architecture.yaml");
        let diagram = dendryform_parse::parse_yaml(yaml).unwrap();
        let plan = compute_layout(&diagram).unwrap();
        let svg = render_svg(&plan, &Theme::dark(), 1100.0).unwrap();

        assert!(svg.contains("<svg"));
        assert!(svg.contains("ai-kasu"));
        assert!(svg.contains("MCP server architecture"));
        assert!(svg.contains("Claude Code"));
        assert!(svg.contains("KnowledgeEngine"));
        assert!(svg.contains("SERVER"));
    }

    #[test]
    fn test_render_oxur_lisp_svg() {
        let yaml = include_str!("../../../examples/oxur-lisp/architecture.yaml");
        let diagram = dendryform_parse::parse_yaml(yaml).unwrap();
        let plan = compute_layout(&diagram).unwrap();
        let svg = render_svg(&plan, &Theme::dark(), 1100.0).unwrap();

        assert!(svg.contains("<svg"));
        assert!(svg.contains("oxur"));
        assert!(svg.contains("language architecture"));
        assert!(svg.contains("OXUR COMPILATION PIPELINE"));
        assert!(svg.contains("Frontend (oxur-lang)"));
    }
}
