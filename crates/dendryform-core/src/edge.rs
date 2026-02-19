//! Semantic edge (relationship) type.

use serde::{Deserialize, Serialize};

use crate::id::NodeId;
use crate::kind::EdgeKind;

/// A semantic relationship between two nodes.
///
/// Edges are not rendered visually in HTML (use connectors and flow labels
/// for visual connections). They are used by Structurizr and Mermaid
/// exporters to generate relationship diagrams.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Edge {
    from: NodeId,
    to: NodeId,
    kind: EdgeKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

impl Edge {
    /// Creates a new edge between two nodes.
    pub fn new(from: NodeId, to: NodeId, kind: EdgeKind) -> Self {
        Self {
            from,
            to,
            kind,
            label: None,
        }
    }

    /// Creates a new edge with a descriptive label.
    pub fn with_label(from: NodeId, to: NodeId, kind: EdgeKind, label: &str) -> Self {
        Self {
            from,
            to,
            kind,
            label: Some(label.to_owned()),
        }
    }

    /// Returns the source node ID.
    pub fn from_id(&self) -> &NodeId {
        &self.from
    }

    /// Returns the target node ID.
    pub fn to_id(&self) -> &NodeId {
        &self.to
    }

    /// Returns the relationship kind.
    pub fn kind(&self) -> EdgeKind {
        self.kind
    }

    /// Returns the optional label.
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let edge = Edge::new(
            NodeId::new("a").unwrap(),
            NodeId::new("b").unwrap(),
            EdgeKind::Uses,
        );
        assert_eq!(edge.from_id().as_str(), "a");
        assert_eq!(edge.to_id().as_str(), "b");
        assert_eq!(edge.kind(), EdgeKind::Uses);
        assert_eq!(edge.label(), None);
    }

    #[test]
    fn test_with_label() {
        let edge = Edge::with_label(
            NodeId::new("api").unwrap(),
            NodeId::new("db").unwrap(),
            EdgeKind::Reads,
            "SQL queries",
        );
        assert_eq!(edge.label(), Some("SQL queries"));
    }

    #[test]
    fn test_serde_round_trip() {
        let edge = Edge::with_label(
            NodeId::new("api").unwrap(),
            NodeId::new("db").unwrap(),
            EdgeKind::Writes,
            "inserts",
        );
        let json = serde_json::to_string(&edge).unwrap();
        let deserialized: Edge = serde_json::from_str(&json).unwrap();
        assert_eq!(edge, deserialized);
    }

    #[test]
    fn test_serde_omits_none_label() {
        let edge = Edge::new(
            NodeId::new("a").unwrap(),
            NodeId::new("b").unwrap(),
            EdgeKind::Uses,
        );
        let json = serde_json::to_string(&edge).unwrap();
        assert!(!json.contains("label"));
    }
}
