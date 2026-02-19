//! Layer enum — the ordered visual elements of a diagram.

use serde::{Deserialize, Serialize};

use crate::connector::Connector;
use crate::tier::Tier;

/// Directional labels between tiers (typically above external services).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FlowLabels {
    items: Vec<String>,
}

impl FlowLabels {
    /// Creates new flow labels.
    pub fn new(items: Vec<String>) -> Self {
        Self { items }
    }

    /// Returns the label items.
    pub fn items(&self) -> &[String] {
        &self.items
    }
}

/// A single visual element in the diagram's layer stack.
///
/// Layers are rendered in order from top to bottom. Each layer is
/// exactly one of: a tier (horizontal band of nodes), a connector
/// (visual link between tiers), or flow labels (directional arrows).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Layer {
    /// A horizontal band of nodes.
    Tier(Tier),
    /// A visual connector between adjacent tiers.
    Connector(Connector),
    /// Directional labels between tiers.
    FlowLabels(FlowLabels),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connector::ConnectorStyle;
    use crate::id::NodeId;

    #[test]
    fn test_flow_labels() {
        let labels = FlowLabels::new(vec!["SQL queries".to_owned(), "cache reads".to_owned()]);
        assert_eq!(labels.items().len(), 2);
    }

    #[test]
    fn test_layer_variants() {
        let tier = Tier::new(NodeId::new("test").unwrap(), vec![]);
        let layer = Layer::Tier(tier);
        assert!(matches!(layer, Layer::Tier(_)));

        let conn = Connector::new(ConnectorStyle::Dots);
        let layer = Layer::Connector(conn);
        assert!(matches!(layer, Layer::Connector(_)));

        let labels = FlowLabels::new(vec!["test".to_owned()]);
        let layer = Layer::FlowLabels(labels);
        assert!(matches!(layer, Layer::FlowLabels(_)));
    }
}
