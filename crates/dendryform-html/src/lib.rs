//! # dendryform-html
//!
//! Responsive HTML renderer for dendryform diagrams.
//!
//! Consumes a [`LayoutPlan`](dendryform_layout::LayoutPlan) and
//! [`Theme`](dendryform_core::Theme) to produce a self-contained HTML
//! file with embedded CSS. The output is dark-themed, responsive, and
//! interactive with hover states and animations.
//!
//! ## Quick Start
//!
//! ```no_run
//! use dendryform_core::Theme;
//! use dendryform_html::render_html;
//! use dendryform_layout::compute_layout;
//!
//! let diagram = dendryform_parse::parse_yaml_file("examples/taproot/taproot.yml").unwrap();
//! let plan = compute_layout(&diagram).unwrap();
//! let html = render_html(&plan, &Theme::dark()).unwrap();
//! std::fs::write("output.html", html).unwrap();
//! ```

mod css;
mod error;
mod render;

pub use error::RenderError;
pub use render::render_html;

/// Returns the version of the dendryform-html crate.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use dendryform_core::{
        Color, Connector, ConnectorStyle, Container, ContainerBorder, Diagram, DiagramHeader,
        FlowLabels, Layer, LegendEntry, Node, NodeId, NodeKind, RawDiagram, Theme, Tier,
        TierLayout, Title,
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
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("</html>"));
        assert!(html.contains("accent"));
        assert!(html.contains("test"));
    }

    #[test]
    fn test_render_contains_css_variables() {
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![test_node("app", Color::Blue)],
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("--bg: #0a0e14"));
        assert!(html.contains("--text: #c4cdd9"));
        assert!(html.contains("--accent-blue: #4fc3f7"));
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
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("class=\"node green"));
        assert!(html.contains("myapp"));
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
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("protocol-label"));
        assert!(html.contains("HTTPS"));
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
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("container-solid"));
        assert!(html.contains("container-label"));
        assert!(html.contains("server"));
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
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("legend"));
        assert!(html.contains("swatch blue"));
        assert!(html.contains("Clients"));
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
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("flow-labels"));
        assert!(html.contains("SQL queries"));
        assert!(html.contains("\u{2193}")); // down arrow
    }

    #[test]
    fn test_render_grid_layout() {
        let mut tier = Tier::new(
            NodeId::new("grid").unwrap(),
            vec![
                test_node("a", Color::Blue),
                test_node("b", Color::Blue),
                test_node("c", Color::Blue),
                test_node("d", Color::Blue),
            ],
        );
        tier.set_layout(TierLayout::Grid { columns: 4 });
        let diagram = make_diagram(vec![Layer::Tier(tier)], vec![]);
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("grid-4"));
    }

    #[test]
    fn test_render_responsive_css() {
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![test_node("a", Color::Blue)],
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("@media (max-width: 800px)"));
    }

    #[test]
    fn test_render_animations() {
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![test_node("a", Color::Blue)],
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("@keyframes fadeIn"));
        assert!(html.contains("@keyframes slideUp"));
    }

    #[test]
    fn test_render_no_animations_when_disabled() {
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![test_node("a", Color::Blue)],
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let mut theme = Theme::dark();
        let overrides = dendryform_core::ThemeOverrides {
            animate: Some(false),
            ..Default::default()
        };
        theme = theme.merge(overrides);
        let html = render_html(&plan, &theme).unwrap();

        assert!(html.contains("animation: none !important"));
        assert!(!html.contains("@keyframes fadeIn"));
    }

    #[test]
    fn test_render_taproot_full() {
        let yaml = include_str!("../../../examples/taproot/taproot.yml");
        let diagram = dendryform_parse::parse_yaml(yaml).unwrap();
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        // Structure checks
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("taproot"));
        assert!(html.contains("system architecture"));
        assert!(html.contains("Claude Desktop"));
        assert!(html.contains("Streamable HTTP + SSE"));
        assert!(html.contains("container-solid"));
        assert!(html.contains("container-dashed"));
        assert!(html.contains("knowledge engine"));
        assert!(html.contains("BigQuery"));
        assert!(html.contains("flow-labels"));
        assert!(html.contains("SQL queries"));
        assert!(html.contains("legend"));
        assert!(html.contains("Client / Auth"));
    }

    #[test]
    fn test_render_ai_kasu_full() {
        let yaml = include_str!("../../../examples/ai-kasu/architecture.yaml");
        let diagram = dendryform_parse::parse_yaml(yaml).unwrap();
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("ai-kasu"));
        assert!(html.contains("MCP server architecture"));
        assert!(html.contains("Claude Code"));
        assert!(html.contains("KnowledgeEngine"));
        assert!(html.contains("container-solid"));
        assert!(html.contains("container-dashed"));
        assert!(html.contains("knowledge engine"));
        assert!(html.contains("flow-labels"));
        assert!(html.contains("legend"));
        assert!(html.contains("Tool Registries"));
    }

    #[test]
    fn test_render_oxur_lisp_full() {
        let yaml = include_str!("../../../examples/oxur-lisp/architecture.yaml");
        let diagram = dendryform_parse::parse_yaml(yaml).unwrap();
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("oxur"));
        assert!(html.contains("language architecture"));
        assert!(html.contains("Stage 1: Parse"));
        assert!(html.contains("container-solid"));
        assert!(html.contains("container-dashed"));
        assert!(html.contains("oxur compilation pipeline"));
        assert!(html.contains("flow-labels"));
        assert!(html.contains("legend"));
        assert!(html.contains("Frontend (oxur-lang)"));
    }

    #[test]
    fn test_html_escaping() {
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
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("&amp;"));
        assert!(html.contains("&lt;"));
        assert!(html.contains("&gt;"));
        // Must not contain unescaped
        assert!(!html.contains("A & B"));
        assert!(!html.contains("x < y"));
    }
}
