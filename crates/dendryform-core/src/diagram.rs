//! Diagram types — the top-level document structure.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::edge::Edge;
use crate::error::ValidationError;
use crate::id::NodeId;
use crate::layer::Layer;
use crate::legend::LegendEntry;

/// Maximum allowed container nesting depth.
const MAX_NESTING_DEPTH: usize = 3;

/// The title block of a diagram.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Title {
    /// The main title text (appears after the accent word).
    text: String,
    /// The accent word rendered in the accent color.
    accent: String,
}

impl Title {
    /// Creates a new title.
    pub fn new(text: &str, accent: &str) -> Self {
        Self {
            text: text.to_owned(),
            accent: accent.to_owned(),
        }
    }

    /// Returns the main title text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the accent word.
    pub fn accent(&self) -> &str {
        &self.accent
    }
}

/// Diagram metadata and theme configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DiagramHeader {
    title: Title,
    subtitle: String,
    theme: String,
}

impl DiagramHeader {
    /// Creates a new diagram header.
    pub fn new(title: Title, subtitle: &str, theme: &str) -> Self {
        Self {
            title,
            subtitle: subtitle.to_owned(),
            theme: theme.to_owned(),
        }
    }

    /// Returns the title block.
    pub fn title(&self) -> &Title {
        &self.title
    }

    /// Returns the subtitle.
    pub fn subtitle(&self) -> &str {
        &self.subtitle
    }

    /// Returns the theme identifier.
    pub fn theme(&self) -> &str {
        &self.theme
    }
}

/// The raw, unchecked deserialization target for a diagram YAML/JSON file.
///
/// Serde populates this directly. Use [`Diagram::try_from`] to validate
/// invariants and produce a checked [`Diagram`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RawDiagram {
    /// Diagram metadata.
    pub diagram: DiagramHeader,
    /// Ordered visual layers.
    pub layers: Vec<Layer>,
    /// Legend entries.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub legend: Vec<LegendEntry>,
    /// Semantic edges.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edges: Vec<Edge>,
}

/// A validated diagram that cannot represent an invalid state.
///
/// Invariants enforced:
/// - No duplicate node IDs across the entire diagram
/// - All edge `from`/`to` references point to existing node IDs
/// - No empty tiers (every tier has nodes or a container)
/// - Container nesting depth does not exceed the maximum
///
/// Construct via `Diagram::try_from(raw_diagram)`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Diagram(RawDiagram);

impl Diagram {
    /// Returns the diagram header (title, subtitle, theme).
    pub fn header(&self) -> &DiagramHeader {
        &self.0.diagram
    }

    /// Returns the ordered visual layers.
    pub fn layers(&self) -> &[Layer] {
        &self.0.layers
    }

    /// Returns the legend entries.
    pub fn legend(&self) -> &[LegendEntry] {
        &self.0.legend
    }

    /// Returns the semantic edges.
    pub fn edges(&self) -> &[Edge] {
        &self.0.edges
    }

    /// Collects all node IDs from the diagram (including nested containers).
    fn collect_all_node_ids(layers: &[Layer]) -> Vec<&NodeId> {
        let mut ids = Vec::new();
        Self::collect_node_ids_recursive(layers, &mut ids);
        ids
    }

    fn collect_node_ids_recursive<'a>(layers: &'a [Layer], ids: &mut Vec<&'a NodeId>) {
        for layer in layers {
            if let Layer::Tier(tier) = layer {
                for node in tier.nodes() {
                    ids.push(node.id());
                }
                if let Some(container) = tier.container() {
                    Self::collect_node_ids_recursive(container.layers(), ids);
                }
            }
        }
    }

    /// Validates that no empty tiers exist (including in nested containers).
    fn check_empty_tiers(layers: &[Layer]) -> Result<(), ValidationError> {
        for layer in layers {
            if let Layer::Tier(tier) = layer {
                if tier.is_empty() {
                    return Err(ValidationError::EmptyTier {
                        id: tier.id().to_string(),
                    });
                }
                if let Some(container) = tier.container() {
                    Self::check_empty_tiers(container.layers())?;
                }
            }
        }
        Ok(())
    }

    /// Validates container nesting depth.
    fn check_nesting_depth(layers: &[Layer], current_depth: usize) -> Result<(), ValidationError> {
        for layer in layers {
            if let Layer::Tier(tier) = layer {
                if let Some(container) = tier.container() {
                    let depth = current_depth + 1;
                    if depth > MAX_NESTING_DEPTH {
                        return Err(ValidationError::NestingTooDeep {
                            max_depth: MAX_NESTING_DEPTH,
                            actual_depth: depth,
                        });
                    }
                    Self::check_nesting_depth(container.layers(), depth)?;
                }
            }
        }
        Ok(())
    }
}

impl TryFrom<RawDiagram> for Diagram {
    type Error = ValidationError;

    fn try_from(raw: RawDiagram) -> Result<Self, Self::Error> {
        // Check for empty tiers.
        Self::check_empty_tiers(&raw.layers)?;

        // Check nesting depth.
        Self::check_nesting_depth(&raw.layers, 0)?;

        // Collect all node IDs and check for duplicates.
        let all_ids = Self::collect_all_node_ids(&raw.layers);
        let mut seen = HashSet::new();
        for id in &all_ids {
            if !seen.insert(id.as_str()) {
                return Err(ValidationError::DuplicateNodeId { id: id.to_string() });
            }
        }

        // Check that all edge references point to existing nodes.
        for edge in &raw.edges {
            if !seen.contains(edge.from_id().as_str()) {
                return Err(ValidationError::DanglingEdgeReference {
                    id: edge.from_id().to_string(),
                    field: "from",
                });
            }
            if !seen.contains(edge.to_id().as_str()) {
                return Err(ValidationError::DanglingEdgeReference {
                    id: edge.to_id().to_string(),
                    field: "to",
                });
            }
        }

        Ok(Self(raw))
    }
}

impl<'de> Deserialize<'de> for Diagram {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = RawDiagram::deserialize(deserializer)?;
        Diagram::try_from(raw).map_err(serde::de::Error::custom)
    }
}

// Helper to build test fixtures without repeating boilerplate.
#[cfg(test)]
fn test_node(id: &str) -> crate::node::Node {
    crate::node::Node::builder()
        .id(NodeId::new(id).unwrap())
        .kind(crate::kind::NodeKind::System)
        .color(crate::color::Color::Blue)
        .icon("◇")
        .title(id)
        .description("test node")
        .build()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::connector::{Connector, ConnectorStyle};
    use crate::container::{Container, ContainerBorder};
    use crate::kind::EdgeKind;
    use crate::layer::Layer;
    use crate::node::Node;
    use crate::tier::Tier;

    fn simple_raw(nodes: Vec<Node>, edges: Vec<Edge>) -> RawDiagram {
        RawDiagram {
            diagram: DiagramHeader::new(Title::new("test", "test"), "a test diagram", "dark"),
            layers: vec![Layer::Tier(Tier::new(NodeId::new("main").unwrap(), nodes))],
            legend: vec![],
            edges,
        }
    }

    #[test]
    fn test_valid_diagram() {
        let raw = simple_raw(
            vec![test_node("app"), test_node("db")],
            vec![Edge::new(
                NodeId::new("app").unwrap(),
                NodeId::new("db").unwrap(),
                EdgeKind::Uses,
            )],
        );
        let diagram = Diagram::try_from(raw);
        assert!(diagram.is_ok());
    }

    #[test]
    fn test_duplicate_node_id_rejected() {
        let raw = simple_raw(vec![test_node("app"), test_node("app")], vec![]);
        let err = Diagram::try_from(raw).unwrap_err();
        assert!(matches!(err, ValidationError::DuplicateNodeId { id } if id == "app"));
    }

    #[test]
    fn test_dangling_edge_from_rejected() {
        let raw = simple_raw(
            vec![test_node("app")],
            vec![Edge::new(
                NodeId::new("ghost").unwrap(),
                NodeId::new("app").unwrap(),
                EdgeKind::Uses,
            )],
        );
        let err = Diagram::try_from(raw).unwrap_err();
        assert!(
            matches!(err, ValidationError::DanglingEdgeReference { id, field } if id == "ghost" && field == "from")
        );
    }

    #[test]
    fn test_dangling_edge_to_rejected() {
        let raw = simple_raw(
            vec![test_node("app")],
            vec![Edge::new(
                NodeId::new("app").unwrap(),
                NodeId::new("ghost").unwrap(),
                EdgeKind::Uses,
            )],
        );
        let err = Diagram::try_from(raw).unwrap_err();
        assert!(
            matches!(err, ValidationError::DanglingEdgeReference { id, field } if id == "ghost" && field == "to")
        );
    }

    #[test]
    fn test_empty_tier_rejected() {
        let raw = RawDiagram {
            diagram: DiagramHeader::new(Title::new("test", "test"), "test", "dark"),
            layers: vec![Layer::Tier(Tier::new(
                NodeId::new("empty").unwrap(),
                vec![],
            ))],
            legend: vec![],
            edges: vec![],
        };
        let err = Diagram::try_from(raw).unwrap_err();
        assert!(matches!(err, ValidationError::EmptyTier { id } if id == "empty"));
    }

    #[test]
    fn test_tier_with_container_not_empty() {
        let container = Container::new(
            "server",
            ContainerBorder::Solid,
            Color::Green,
            vec![Layer::Tier(Tier::new(
                NodeId::new("inner").unwrap(),
                vec![test_node("api")],
            ))],
        );
        let raw = RawDiagram {
            diagram: DiagramHeader::new(Title::new("test", "test"), "test", "dark"),
            layers: vec![Layer::Tier(Tier::with_container(
                NodeId::new("server").unwrap(),
                container,
            ))],
            legend: vec![],
            edges: vec![],
        };
        assert!(Diagram::try_from(raw).is_ok());
    }

    #[test]
    fn test_connector_layer_does_not_affect_validation() {
        let raw = RawDiagram {
            diagram: DiagramHeader::new(Title::new("test", "test"), "test", "dark"),
            layers: vec![
                Layer::Tier(Tier::new(NodeId::new("top").unwrap(), vec![test_node("a")])),
                Layer::Connector(Connector::with_label(ConnectorStyle::Line, "HTTPS")),
                Layer::Tier(Tier::new(
                    NodeId::new("bottom").unwrap(),
                    vec![test_node("b")],
                )),
            ],
            legend: vec![],
            edges: vec![],
        };
        assert!(Diagram::try_from(raw).is_ok());
    }

    #[test]
    fn test_serde_round_trip() {
        let raw = simple_raw(vec![test_node("app")], vec![]);
        let diagram = Diagram::try_from(raw).unwrap();
        let json = serde_json::to_string_pretty(&diagram).unwrap();
        let deserialized: Diagram = serde_json::from_str(&json).unwrap();
        assert_eq!(diagram, deserialized);
    }

    #[test]
    fn test_nesting_too_deep_rejected() {
        // Build 5-deep nesting to exceed MAX_NESTING_DEPTH = 3.
        // Depths: top(1) -> l1t(2) -> l2t(3) -> l3t(4) — depth 4 > 3, triggers error.
        let deepest_tier = Tier::new(NodeId::new("deep").unwrap(), vec![test_node("d")]);
        let level4 = Container::new(
            "l4",
            ContainerBorder::Dashed,
            Color::Blue,
            vec![Layer::Tier(deepest_tier)],
        );
        let level3 = Container::new(
            "l3",
            ContainerBorder::Dashed,
            Color::Blue,
            vec![Layer::Tier(Tier::with_container(
                NodeId::new("l3t").unwrap(),
                level4,
            ))],
        );
        let level2 = Container::new(
            "l2",
            ContainerBorder::Dashed,
            Color::Blue,
            vec![Layer::Tier(Tier::with_container(
                NodeId::new("l2t").unwrap(),
                level3,
            ))],
        );
        let level1 = Container::new(
            "l1",
            ContainerBorder::Dashed,
            Color::Blue,
            vec![Layer::Tier(Tier::with_container(
                NodeId::new("l1t").unwrap(),
                level2,
            ))],
        );
        let top_tier = Tier::with_container(NodeId::new("top").unwrap(), level1);

        let raw = RawDiagram {
            diagram: DiagramHeader::new(Title::new("test", "test"), "test", "dark"),
            layers: vec![Layer::Tier(top_tier)],
            legend: vec![],
            edges: vec![],
        };
        let err = Diagram::try_from(raw).unwrap_err();
        assert!(matches!(err, ValidationError::NestingTooDeep { .. }));
    }

    #[test]
    fn test_diagram_accessors() {
        let raw = simple_raw(
            vec![test_node("app")],
            vec![Edge::new(
                NodeId::new("app").unwrap(),
                NodeId::new("app").unwrap(),
                EdgeKind::Uses,
            )],
        );
        let diagram = Diagram::try_from(raw).unwrap();

        assert_eq!(diagram.header().title().text(), "test");
        assert_eq!(diagram.header().title().accent(), "test");
        assert_eq!(diagram.header().subtitle(), "a test diagram");
        assert_eq!(diagram.header().theme(), "dark");
        assert_eq!(diagram.layers().len(), 1);
        assert_eq!(diagram.edges().len(), 1);
        assert!(diagram.legend().is_empty());
    }

    #[test]
    fn test_nested_container_node_ids_collected() {
        let container = Container::new(
            "server",
            ContainerBorder::Solid,
            Color::Green,
            vec![Layer::Tier(Tier::new(
                NodeId::new("inner").unwrap(),
                vec![test_node("api")],
            ))],
        );
        let raw = RawDiagram {
            diagram: DiagramHeader::new(Title::new("test", "test"), "test", "dark"),
            layers: vec![Layer::Tier(Tier::with_container(
                NodeId::new("server").unwrap(),
                container,
            ))],
            legend: vec![],
            edges: vec![Edge::new(
                NodeId::new("api").unwrap(),
                NodeId::new("api").unwrap(),
                EdgeKind::Uses,
            )],
        };
        // Edge referencing nested "api" should pass validation
        assert!(Diagram::try_from(raw).is_ok());
    }
}
