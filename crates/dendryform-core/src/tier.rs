//! Tier type — a horizontal band of nodes.

use serde::{Deserialize, Serialize};

use crate::container::Container;
use crate::id::NodeId;
use crate::layout::TierLayout;
use crate::node::Node;

/// A horizontal band of nodes in the diagram, optionally wrapped in a container.
///
/// Each tier has a unique ID, an optional label, a layout hint, and
/// either a list of nodes or a container (or both).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Tier {
    id: NodeId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(default, skip_serializing_if = "is_auto")]
    layout: TierLayout,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    nodes: Vec<Node>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    container: Option<Container>,
}

fn is_auto(layout: &TierLayout) -> bool {
    *layout == TierLayout::Auto
}

impl Tier {
    /// Creates a new tier with nodes.
    pub fn new(id: NodeId, nodes: Vec<Node>) -> Self {
        Self {
            id,
            label: None,
            layout: TierLayout::default(),
            nodes,
            container: None,
        }
    }

    /// Creates a new tier with a container.
    pub fn with_container(id: NodeId, container: Container) -> Self {
        Self {
            id,
            label: None,
            layout: TierLayout::default(),
            nodes: Vec::new(),
            container: Some(container),
        }
    }

    /// Returns the tier's unique identifier.
    pub fn id(&self) -> &NodeId {
        &self.id
    }

    /// Returns the optional tier heading label.
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Sets the tier heading label.
    pub fn set_label(&mut self, label: &str) {
        self.label = Some(label.to_owned());
    }

    /// Returns the layout configuration.
    pub fn layout(&self) -> &TierLayout {
        &self.layout
    }

    /// Sets the layout configuration.
    pub fn set_layout(&mut self, layout: TierLayout) {
        self.layout = layout;
    }

    /// Returns the nodes in this tier.
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    /// Returns the optional container.
    pub fn container(&self) -> Option<&Container> {
        self.container.as_ref()
    }

    /// Returns `true` if this tier has neither nodes nor a container.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty() && self.container.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::kind::NodeKind;

    #[test]
    fn test_new_with_nodes() {
        let node = Node::builder()
            .id(NodeId::new("app").unwrap())
            .kind(NodeKind::System)
            .color(Color::Blue)
            .icon("◇")
            .title("App")
            .description("The app")
            .build()
            .unwrap();

        let tier = Tier::new(NodeId::new("clients").unwrap(), vec![node]);
        assert_eq!(tier.id().as_str(), "clients");
        assert_eq!(tier.nodes().len(), 1);
        assert!(!tier.is_empty());
    }

    #[test]
    fn test_empty_tier() {
        let tier = Tier::new(NodeId::new("empty").unwrap(), vec![]);
        assert!(tier.is_empty());
    }
}
