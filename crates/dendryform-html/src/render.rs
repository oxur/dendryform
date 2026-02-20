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

#[cfg(test)]
mod tests {
    use super::*;
    use dendryform_core::{
        Color, Connector, ConnectorStyle, Container, ContainerBorder, Diagram, DiagramHeader,
        FlowLabels, Layer, LegendEntry, Node, NodeId, NodeKind, RawDiagram, Tech, Tier, TierLayout,
        Title,
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

    fn test_node_with_tech(id: &str, color: Color, techs: Vec<&str>) -> Node {
        Node::builder()
            .id(NodeId::new(id).unwrap())
            .kind(NodeKind::System)
            .color(color)
            .icon("\u{25c7}")
            .title(id)
            .description("test node with tech")
            .tech(techs.into_iter().map(Tech::new).collect())
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
    fn test_escape_html_function() {
        assert_eq!(escape_html("a & b"), "a &amp; b");
        assert_eq!(escape_html("<tag>"), "&lt;tag&gt;");
        assert_eq!(escape_html("a \"b\""), "a &quot;b&quot;");
        assert_eq!(escape_html("no special"), "no special");
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
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("internal-connector"));
        assert!(html.contains("dot"));
    }

    #[test]
    fn test_render_nested_dashed_container() {
        let inner_container = Container::new(
            "inner-service",
            ContainerBorder::Dashed,
            Color::Purple,
            vec![Layer::Tier(Tier::new(
                NodeId::new("deep").unwrap(),
                vec![test_node("deep-api", Color::Purple)],
            ))],
        );
        let outer_container = Container::new(
            "outer-server",
            ContainerBorder::Solid,
            Color::Green,
            vec![Layer::Tier(Tier::with_container(
                NodeId::new("inner-tier").unwrap(),
                inner_container,
            ))],
        );
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::with_container(
                NodeId::new("outer").unwrap(),
                outer_container,
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("container-solid"));
        assert!(html.contains("container-dashed"));
        assert!(html.contains("outer-server"));
        assert!(html.contains("inner-service"));
        assert!(html.contains("deep-api"));
    }

    #[test]
    fn test_render_node_with_tech() {
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![test_node_with_tech(
                    "app",
                    Color::Blue,
                    vec!["Rust", "axum"],
                )],
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("node-tech"));
        assert!(html.contains("Rust"));
        assert!(html.contains("axum"));
    }

    #[test]
    fn test_render_single_node_layout() {
        let mut tier = Tier::new(
            NodeId::new("main").unwrap(),
            vec![test_node("app", Color::Blue)],
        );
        tier.set_layout(TierLayout::Single);
        let diagram = make_diagram(vec![Layer::Tier(tier)], vec![]);
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("client-node"));
    }

    #[test]
    fn test_render_connector_without_label() {
        let diagram = make_diagram(
            vec![
                Layer::Tier(Tier::new(
                    NodeId::new("top").unwrap(),
                    vec![test_node("a", Color::Blue)],
                )),
                Layer::Connector(Connector::new(ConnectorStyle::Line)),
                Layer::Tier(Tier::new(
                    NodeId::new("bottom").unwrap(),
                    vec![test_node("b", Color::Green)],
                )),
            ],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("class=\"connector\""));
        assert!(html.contains("class=\"line\""));
        // No protocol label element (CSS always defines the class, but no element is emitted)
        assert!(!html.contains("<div class=\"protocol-label\">"));
    }

    #[test]
    fn test_render_empty_legend() {
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![test_node("a", Color::Blue)],
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        // Legend div should not be present
        assert!(!html.contains("class=\"legend\""));
    }

    #[test]
    fn test_render_tier_with_label() {
        let mut tier = Tier::new(
            NodeId::new("main").unwrap(),
            vec![test_node("a", Color::Blue)],
        );
        tier.set_label("My Section");
        let diagram = make_diagram(vec![Layer::Tier(tier)], vec![]);
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("tier-label"));
        assert!(html.contains("My Section"));
    }

    #[test]
    fn test_render_container_with_flow_labels() {
        let container = Container::new(
            "server",
            ContainerBorder::Solid,
            Color::Green,
            vec![
                Layer::Tier(Tier::new(
                    NodeId::new("top").unwrap(),
                    vec![test_node("a", Color::Green)],
                )),
                Layer::FlowLabels(FlowLabels::new(vec!["queries".to_owned()])),
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
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("flow-labels"));
        assert!(html.contains("queries"));
    }

    #[test]
    fn test_render_container_with_tier_label() {
        let container = Container::new(
            "server",
            ContainerBorder::Solid,
            Color::Green,
            vec![{
                let mut t = Tier::new(
                    NodeId::new("inner").unwrap(),
                    vec![test_node("api", Color::Green)],
                );
                t.set_label("Inner Label");
                Layer::Tier(t)
            }],
        );
        let diagram = make_diagram(
            vec![{
                let mut t = Tier::with_container(NodeId::new("outer").unwrap(), container);
                t.set_label("Outer Label");
                Layer::Tier(t)
            }],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("Outer Label"));
        assert!(html.contains("Inner Label"));
    }

    #[test]
    fn test_render_legend_with_entries() {
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::new(
                NodeId::new("main").unwrap(),
                vec![test_node("a", Color::Blue)],
            ))],
            vec![
                LegendEntry::new(Color::Blue, "Clients"),
                LegendEntry::new(Color::Green, "Servers"),
                LegendEntry::new(Color::Amber, "Data"),
            ],
        );
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("class=\"legend\""));
        assert!(html.contains("Clients"));
        assert!(html.contains("Servers"));
        assert!(html.contains("Data"));
        assert!(html.contains("swatch blue"));
        assert!(html.contains("swatch green"));
        assert!(html.contains("swatch amber"));
    }

    #[test]
    fn test_render_connector_with_label() {
        let diagram = make_diagram(
            vec![
                Layer::Tier(Tier::new(
                    NodeId::new("top").unwrap(),
                    vec![test_node("a", Color::Blue)],
                )),
                Layer::Connector(Connector::with_label(ConnectorStyle::Line, "gRPC")),
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
        assert!(html.contains("gRPC"));
    }

    #[test]
    fn test_render_top_level_flow_labels() {
        let diagram = make_diagram(
            vec![
                Layer::Tier(Tier::new(
                    NodeId::new("top").unwrap(),
                    vec![test_node("a", Color::Blue)],
                )),
                Layer::FlowLabels(FlowLabels::new(vec![
                    "SQL queries".to_owned(),
                    "REST calls".to_owned(),
                ])),
                Layer::Tier(Tier::new(
                    NodeId::new("bottom").unwrap(),
                    vec![test_node("b", Color::Green)],
                )),
            ],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("flow-labels"));
        assert!(html.contains("SQL queries"));
        assert!(html.contains("REST calls"));
        assert!(html.contains("\u{2193}")); // down arrow
    }

    #[test]
    fn test_render_container_without_parent_label() {
        let container = Container::new(
            "server",
            ContainerBorder::Solid,
            Color::Green,
            vec![Layer::Tier(Tier::new(
                NodeId::new("inner").unwrap(),
                vec![test_node("api", Color::Green)],
            ))],
        );
        // No parent tier label set
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::with_container(
                NodeId::new("outer").unwrap(),
                container,
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("container-solid"));
        assert!(html.contains("server"));
    }

    #[test]
    fn test_render_multi_column_grid() {
        let mut tier = Tier::new(
            NodeId::new("grid").unwrap(),
            vec![
                test_node("a", Color::Blue),
                test_node("b", Color::Green),
                test_node("c", Color::Purple),
            ],
        );
        tier.set_layout(TierLayout::Grid { columns: 3 });
        let diagram = make_diagram(vec![Layer::Tier(tier)], vec![]);
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("grid-3"));
        assert!(html.contains("a"));
        assert!(html.contains("b"));
        assert!(html.contains("c"));
    }

    #[test]
    fn test_render_nested_container_with_connector() {
        let container = Container::new(
            "server",
            ContainerBorder::Solid,
            Color::Green,
            vec![
                Layer::Tier(Tier::new(
                    NodeId::new("top-inner").unwrap(),
                    vec![test_node("api", Color::Green)],
                )),
                Layer::Connector(Connector::new(ConnectorStyle::Dots)),
                Layer::Tier(Tier::new(
                    NodeId::new("bot-inner").unwrap(),
                    vec![test_node("db", Color::Amber)],
                )),
            ],
        );
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::with_container(
                NodeId::new("outer").unwrap(),
                container,
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("internal-connector"));
        assert!(html.contains("dot"));
        assert!(html.contains("api"));
        assert!(html.contains("db"));
    }

    #[test]
    fn test_render_full_example_taproot() {
        let yaml = include_str!("../../../examples/taproot/architecture.yaml");
        let diagram: Diagram = dendryform_parse::parse_yaml(yaml).unwrap();
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("</html>"));
        assert!(html.contains("taproot"));
        assert!(html.contains("Streamable HTTP"));
        // Legend should have entries
        assert!(html.contains("class=\"legend\""));
    }

    #[test]
    fn test_render_full_example_ai_kasu() {
        let yaml = include_str!("../../../examples/ai-kasu/architecture.yaml");
        let diagram: Diagram = dendryform_parse::parse_yaml(yaml).unwrap();
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("ai-kasu"));
        // Should have nested containers
        assert!(html.contains("container-solid"));
        assert!(html.contains("container-dashed"));
    }

    #[test]
    fn test_render_dashed_container() {
        let container = Container::new(
            "internal",
            ContainerBorder::Dashed,
            Color::Purple,
            vec![Layer::Tier(Tier::new(
                NodeId::new("inner").unwrap(),
                vec![test_node("svc", Color::Purple)],
            ))],
        );
        let diagram = make_diagram(
            vec![Layer::Tier(Tier::with_container(
                NodeId::new("outer").unwrap(),
                container,
            ))],
            vec![],
        );
        let plan = compute_layout(&diagram).unwrap();
        let html = render_html(&plan, &Theme::dark()).unwrap();

        assert!(html.contains("container-dashed"));
        assert!(html.contains("internal"));
    }

    #[test]
    fn test_escape_html_all_special_chars() {
        let result = escape_html("a & b < c > d \"e\"");
        assert_eq!(result, "a &amp; b &lt; c &gt; d &quot;e&quot;");
    }
}
