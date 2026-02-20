//! Semantic kind enums for nodes and edges.

use std::fmt;

use serde::{Deserialize, Serialize};

/// The semantic type of a node in the architecture diagram.
///
/// These map to C4 model concepts for export compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum NodeKind {
    /// A human user of the system.
    Person,
    /// A top-level software system.
    System,
    /// A deployable unit within a system.
    Container,
    /// A module or service within a container.
    Component,
    /// Infrastructure (databases, message queues, cloud services).
    Infrastructure,
    /// A logical grouping of related elements.
    Group,
}

impl fmt::Display for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Person => f.write_str("person"),
            Self::System => f.write_str("system"),
            Self::Container => f.write_str("container"),
            Self::Component => f.write_str("component"),
            Self::Infrastructure => f.write_str("infrastructure"),
            Self::Group => f.write_str("group"),
        }
    }
}

/// The semantic type of a relationship between nodes.
///
/// Used by Structurizr and Mermaid exporters to label edges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum EdgeKind {
    /// General dependency or invocation.
    Uses,
    /// Read access to a data source.
    Reads,
    /// Write access to a data source.
    Writes,
    /// Deployment relationship.
    Deploys,
    /// Containment relationship.
    Contains,
}

impl fmt::Display for EdgeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uses => f.write_str("uses"),
            Self::Reads => f.write_str("reads"),
            Self::Writes => f.write_str("writes"),
            Self::Deploys => f.write_str("deploys"),
            Self::Contains => f.write_str("contains"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_kind_display_all() {
        assert_eq!(format!("{}", NodeKind::Person), "person");
        assert_eq!(format!("{}", NodeKind::System), "system");
        assert_eq!(format!("{}", NodeKind::Container), "container");
        assert_eq!(format!("{}", NodeKind::Component), "component");
        assert_eq!(format!("{}", NodeKind::Infrastructure), "infrastructure");
        assert_eq!(format!("{}", NodeKind::Group), "group");
    }

    #[test]
    fn test_edge_kind_display_all() {
        assert_eq!(format!("{}", EdgeKind::Uses), "uses");
        assert_eq!(format!("{}", EdgeKind::Reads), "reads");
        assert_eq!(format!("{}", EdgeKind::Writes), "writes");
        assert_eq!(format!("{}", EdgeKind::Deploys), "deploys");
        assert_eq!(format!("{}", EdgeKind::Contains), "contains");
    }

    #[test]
    fn test_node_kind_serde_all_variants() {
        let variants = vec![
            (NodeKind::Person, "\"person\""),
            (NodeKind::System, "\"system\""),
            (NodeKind::Container, "\"container\""),
            (NodeKind::Component, "\"component\""),
            (NodeKind::Infrastructure, "\"infrastructure\""),
            (NodeKind::Group, "\"group\""),
        ];
        for (kind, expected_json) in variants {
            let json = serde_json::to_string(&kind).unwrap();
            assert_eq!(json, expected_json);
            let deserialized: NodeKind = serde_json::from_str(&json).unwrap();
            assert_eq!(kind, deserialized);
        }
    }

    #[test]
    fn test_edge_kind_serde_all_variants() {
        let variants = vec![
            (EdgeKind::Uses, "\"uses\""),
            (EdgeKind::Reads, "\"reads\""),
            (EdgeKind::Writes, "\"writes\""),
            (EdgeKind::Deploys, "\"deploys\""),
            (EdgeKind::Contains, "\"contains\""),
        ];
        for (kind, expected_json) in variants {
            let json = serde_json::to_string(&kind).unwrap();
            assert_eq!(json, expected_json);
            let deserialized: EdgeKind = serde_json::from_str(&json).unwrap();
            assert_eq!(kind, deserialized);
        }
    }

    #[test]
    fn test_copy() {
        let a = NodeKind::System;
        let b = a;
        assert_eq!(a, b);

        let c = EdgeKind::Writes;
        let d = c;
        assert_eq!(c, d);
    }

    #[test]
    fn test_node_kind_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(NodeKind::Person);
        set.insert(NodeKind::System);
        set.insert(NodeKind::Person); // duplicate
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_edge_kind_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(EdgeKind::Uses);
        set.insert(EdgeKind::Reads);
        set.insert(EdgeKind::Uses); // duplicate
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_node_kind_debug() {
        let debug = format!("{:?}", NodeKind::Infrastructure);
        assert!(debug.contains("Infrastructure"));
    }

    #[test]
    fn test_edge_kind_debug() {
        let debug = format!("{:?}", EdgeKind::Deploys);
        assert!(debug.contains("Deploys"));
    }

    #[test]
    fn test_node_kind_serde_invalid() {
        let result: Result<NodeKind, _> = serde_json::from_str("\"invalid\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_edge_kind_serde_invalid() {
        let result: Result<EdgeKind, _> = serde_json::from_str("\"invalid\"");
        assert!(result.is_err());
    }
}
