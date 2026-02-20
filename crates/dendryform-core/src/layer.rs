//! Layer enum — the ordered visual elements of a diagram.

use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

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
///
/// In YAML, each layer is a map with a single key identifying the variant:
/// ```yaml
/// - tier:
///     id: main
///     nodes: [...]
/// - connector:
///     style: line
/// - flow_labels:
///     items: [...]
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
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

impl<'de> Deserialize<'de> for Layer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(LayerVisitor)
    }
}

struct LayerVisitor;

impl<'de> Visitor<'de> for LayerVisitor {
    type Value = Layer;

    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("a map with a single key: \"tier\", \"connector\", or \"flow_labels\"")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let key: String = map
            .next_key()?
            .ok_or_else(|| de::Error::custom("expected a layer variant key"))?;

        let layer = match key.as_str() {
            "tier" => Layer::Tier(map.next_value()?),
            "connector" => Layer::Connector(map.next_value()?),
            "flow_labels" => Layer::FlowLabels(map.next_value()?),
            other => {
                return Err(de::Error::unknown_variant(
                    other,
                    &["tier", "connector", "flow_labels"],
                ));
            }
        };

        Ok(layer)
    }
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

    #[test]
    fn test_yaml_round_trip_tier() {
        let tier = Tier::new(NodeId::new("test").unwrap(), vec![]);
        let layer = Layer::Tier(tier);
        let json = serde_json::to_string(&layer).unwrap();
        let deserialized: Layer = serde_json::from_str(&json).unwrap();
        assert_eq!(layer, deserialized);
    }

    #[test]
    fn test_yaml_round_trip_connector() {
        let conn = Connector::new(ConnectorStyle::Dots);
        let layer = Layer::Connector(conn);
        let json = serde_json::to_string(&layer).unwrap();
        let deserialized: Layer = serde_json::from_str(&json).unwrap();
        assert_eq!(layer, deserialized);
    }

    #[test]
    fn test_yaml_round_trip_flow_labels() {
        let labels = FlowLabels::new(vec!["test".to_owned()]);
        let layer = Layer::FlowLabels(labels);
        let json = serde_json::to_string(&layer).unwrap();
        let deserialized: Layer = serde_json::from_str(&json).unwrap();
        assert_eq!(layer, deserialized);
    }

    #[test]
    fn test_deserialize_unknown_variant() {
        let json = r#"{"unknown": {}}"#;
        let err = serde_json::from_str::<Layer>(json).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("unknown"));
    }

    #[test]
    fn test_deserialize_empty_map_rejected() {
        let json = "{}";
        let err = serde_json::from_str::<Layer>(json).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("expected a layer variant key"));
    }

    #[test]
    fn test_flow_labels_items_accessor() {
        let labels = FlowLabels::new(vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]);
        assert_eq!(labels.items().len(), 3);
        assert_eq!(labels.items()[0], "a");
        assert_eq!(labels.items()[2], "c");
    }

    #[test]
    fn test_flow_labels_serde_round_trip() {
        let labels = FlowLabels::new(vec!["SQL queries".to_owned()]);
        let json = serde_json::to_string(&labels).unwrap();
        let deserialized: FlowLabels = serde_json::from_str(&json).unwrap();
        assert_eq!(labels, deserialized);
    }

    #[test]
    fn test_layer_debug() {
        let labels = FlowLabels::new(vec!["test".to_owned()]);
        let layer = Layer::FlowLabels(labels);
        let debug = format!("{layer:?}");
        assert!(debug.contains("FlowLabels"));
    }

    #[test]
    fn test_layer_clone_eq() {
        let conn = Connector::new(ConnectorStyle::Line);
        let layer = Layer::Connector(conn);
        let cloned = layer.clone();
        assert_eq!(layer, cloned);
    }

    #[test]
    fn test_deserialize_from_non_map_type_rejected() {
        // Try to deserialize from a JSON array (not a map)
        let json = r#"["tier", {}]"#;
        let err = serde_json::from_str::<Layer>(json).unwrap_err();
        let msg = format!("{err}");
        // The expecting() method should describe what was expected
        assert!(
            msg.contains("tier")
                || msg.contains("connector")
                || msg.contains("flow_labels")
                || msg.contains("map")
                || msg.contains("invalid type")
        );
    }

    #[test]
    fn test_deserialize_from_string_rejected() {
        let json = r#""tier""#;
        let err = serde_json::from_str::<Layer>(json).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("invalid type") || msg.contains("map"));
    }

    #[test]
    fn test_flow_labels_empty() {
        let labels = FlowLabels::new(vec![]);
        assert!(labels.items().is_empty());
    }
}
