//! Node type and consuming builder.

use serde::{Deserialize, Serialize};

use crate::color::Color;
use crate::error::ValidationError;
use crate::id::NodeId;
use crate::kind::NodeKind;
use crate::metadata::Metadata;
use crate::tech::Tech;

/// An individual card within a tier.
///
/// Nodes are the primary visual elements of a dendryform diagram. Each node
/// has a colored top-bar, an icon, title, description, and optional technology
/// badges.
///
/// Construct via [`NodeBuilder`] or deserialize from YAML/JSON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Node {
    id: NodeId,
    kind: NodeKind,
    color: Color,
    icon: String,
    title: String,
    description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    tech: Vec<Tech>,
    #[serde(default, skip_serializing_if = "Metadata::is_empty")]
    metadata: Metadata,
}

impl Node {
    /// Creates a new [`NodeBuilder`].
    pub fn builder() -> NodeBuilder {
        NodeBuilder::default()
    }

    /// Returns the node's unique identifier.
    pub fn id(&self) -> &NodeId {
        &self.id
    }

    /// Returns the semantic kind.
    pub fn kind(&self) -> NodeKind {
        self.kind
    }

    /// Returns the accent color.
    pub fn color(&self) -> Color {
        self.color
    }

    /// Returns the icon character.
    pub fn icon(&self) -> &str {
        &self.icon
    }

    /// Returns the title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the technology badges.
    pub fn tech(&self) -> &[Tech] {
        &self.tech
    }

    /// Returns the extensibility metadata.
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

/// Consuming builder for [`Node`] (AP-11).
///
/// All required fields must be set before calling [`build`](NodeBuilder::build).
///
/// # Examples
///
/// ```
/// use dendryform_core::{Node, NodeId, NodeKind, Color, Tech};
///
/// let node = Node::builder()
///     .id(NodeId::new("web-app").unwrap())
///     .kind(NodeKind::System)
///     .color(Color::Blue)
///     .icon("◇")
///     .title("Web Application")
///     .description("Browser-based frontend")
///     .tech(vec![Tech::new("React"), Tech::new("TypeScript")])
///     .build()
///     .unwrap();
///
/// assert_eq!(node.id().as_str(), "web-app");
/// ```
#[derive(Debug, Default)]
pub struct NodeBuilder {
    id: Option<NodeId>,
    kind: Option<NodeKind>,
    color: Option<Color>,
    icon: Option<String>,
    title: Option<String>,
    description: Option<String>,
    tech: Vec<Tech>,
    metadata: Metadata,
}

impl NodeBuilder {
    /// Sets the node ID.
    pub fn id(mut self, id: NodeId) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the semantic kind.
    pub fn kind(mut self, kind: NodeKind) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Sets the accent color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the icon character.
    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_owned());
        self
    }

    /// Sets the title.
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_owned());
        self
    }

    /// Sets the description.
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());
        self
    }

    /// Sets the technology badges.
    pub fn tech(mut self, tech: Vec<Tech>) -> Self {
        self.tech = tech;
        self
    }

    /// Sets the extensibility metadata.
    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Builds the [`Node`], returning an error if any required field is missing.
    pub fn build(self) -> Result<Node, ValidationError> {
        Ok(Node {
            id: self
                .id
                .ok_or(ValidationError::MissingField { field: "id" })?,
            kind: self
                .kind
                .ok_or(ValidationError::MissingField { field: "kind" })?,
            color: self
                .color
                .ok_or(ValidationError::MissingField { field: "color" })?,
            icon: self
                .icon
                .ok_or(ValidationError::MissingField { field: "icon" })?,
            title: self
                .title
                .ok_or(ValidationError::MissingField { field: "title" })?,
            description: self.description.ok_or(ValidationError::MissingField {
                field: "description",
            })?,
            tech: self.tech,
            metadata: self.metadata,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_node() -> Node {
        Node::builder()
            .id(NodeId::new("web-app").unwrap())
            .kind(NodeKind::System)
            .color(Color::Blue)
            .icon("◇")
            .title("Web Application")
            .description("Browser-based frontend")
            .tech(vec![Tech::new("React")])
            .build()
            .unwrap()
    }

    #[test]
    fn test_builder_all_fields() {
        let node = sample_node();
        assert_eq!(node.id().as_str(), "web-app");
        assert_eq!(node.kind(), NodeKind::System);
        assert_eq!(node.color(), Color::Blue);
        assert_eq!(node.icon(), "◇");
        assert_eq!(node.title(), "Web Application");
        assert_eq!(node.description(), "Browser-based frontend");
        assert_eq!(node.tech().len(), 1);
    }

    #[test]
    fn test_builder_missing_field() {
        let result = Node::builder()
            .kind(NodeKind::System)
            .color(Color::Blue)
            .icon("◇")
            .title("Test")
            .description("Test")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_serde_round_trip() {
        let node = sample_node();
        let json = serde_json::to_string_pretty(&node).unwrap();
        let deserialized: Node = serde_json::from_str(&json).unwrap();
        assert_eq!(node, deserialized);
    }
}
