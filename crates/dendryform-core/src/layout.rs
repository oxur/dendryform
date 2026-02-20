//! Tier layout configuration.

use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

/// How nodes within a tier are arranged.
///
/// In YAML, this can be a string (`single`, `auto`) or a map (`grid: { columns: 4 }`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TierLayout {
    /// Full-width, centered single node.
    Single,
    /// CSS grid with a fixed number of columns.
    Grid {
        /// The number of columns in the grid.
        columns: u32,
    },
    /// Automatic layout (one column per node, up to 4).
    #[default]
    Auto,
}

impl<'de> Deserialize<'de> for TierLayout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(TierLayoutVisitor)
    }
}

struct TierLayoutVisitor;

impl<'de> Visitor<'de> for TierLayoutVisitor {
    type Value = TierLayout;

    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\"single\", \"auto\", or a map like {grid: {columns: N}}")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        match value {
            "single" => Ok(TierLayout::Single),
            "auto" => Ok(TierLayout::Auto),
            other => Err(de::Error::unknown_variant(
                other,
                &["single", "auto", "grid"],
            )),
        }
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let key: String = map
            .next_key()?
            .ok_or_else(|| de::Error::custom("expected a layout variant key"))?;

        match key.as_str() {
            "grid" => {
                #[derive(Deserialize)]
                struct GridData {
                    columns: u32,
                }
                let data: GridData = map.next_value()?;
                Ok(TierLayout::Grid {
                    columns: data.columns,
                })
            }
            "single" => {
                // Allow map form: { single: null } or { single: {} }
                let _: serde::de::IgnoredAny = map.next_value()?;
                Ok(TierLayout::Single)
            }
            "auto" => {
                let _: serde::de::IgnoredAny = map.next_value()?;
                Ok(TierLayout::Auto)
            }
            other => Err(de::Error::unknown_variant(
                other,
                &["single", "auto", "grid"],
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_auto() {
        assert_eq!(TierLayout::default(), TierLayout::Auto);
    }

    #[test]
    fn test_serde_single() {
        let layout = TierLayout::Single;
        let json = serde_json::to_string(&layout).unwrap();
        assert_eq!(json, "\"single\"");
        let deserialized: TierLayout = serde_json::from_str(&json).unwrap();
        assert_eq!(layout, deserialized);
    }

    #[test]
    fn test_serde_grid() {
        let layout = TierLayout::Grid { columns: 3 };
        let json = serde_json::to_string(&layout).unwrap();
        let deserialized: TierLayout = serde_json::from_str(&json).unwrap();
        assert_eq!(layout, deserialized);
    }

    #[test]
    fn test_yaml_single_string() {
        let yaml = "single";
        let layout: TierLayout = serde_yml::from_str(yaml).unwrap();
        assert_eq!(layout, TierLayout::Single);
    }

    #[test]
    fn test_yaml_auto_string() {
        let yaml = "auto";
        let layout: TierLayout = serde_yml::from_str(yaml).unwrap();
        assert_eq!(layout, TierLayout::Auto);
    }

    #[test]
    fn test_yaml_grid_map() {
        let yaml = "grid:\n  columns: 4";
        let layout: TierLayout = serde_yml::from_str(yaml).unwrap();
        assert_eq!(layout, TierLayout::Grid { columns: 4 });
    }

    #[test]
    fn test_yaml_single_map_form() {
        let yaml = "single: ~";
        let layout: TierLayout = serde_yml::from_str(yaml).unwrap();
        assert_eq!(layout, TierLayout::Single);
    }

    #[test]
    fn test_yaml_auto_map_form() {
        let yaml = "auto: ~";
        let layout: TierLayout = serde_yml::from_str(yaml).unwrap();
        assert_eq!(layout, TierLayout::Auto);
    }

    #[test]
    fn test_yaml_unknown_string_variant() {
        let yaml = "\"unknown_variant\"";
        let result: Result<TierLayout, _> = serde_yml::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_yaml_unknown_map_variant() {
        let yaml = "unknown_key:\n  foo: bar";
        let result: Result<TierLayout, _> = serde_yml::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_json_single_string() {
        let json = "\"single\"";
        let layout: TierLayout = serde_json::from_str(json).unwrap();
        assert_eq!(layout, TierLayout::Single);
    }

    #[test]
    fn test_json_auto_string() {
        let json = "\"auto\"";
        let layout: TierLayout = serde_json::from_str(json).unwrap();
        assert_eq!(layout, TierLayout::Auto);
    }

    #[test]
    fn test_json_grid_map() {
        let json = r#"{"grid":{"columns":3}}"#;
        let layout: TierLayout = serde_json::from_str(json).unwrap();
        assert_eq!(layout, TierLayout::Grid { columns: 3 });
    }

    #[test]
    fn test_debug() {
        let layout = TierLayout::Grid { columns: 2 };
        let debug = format!("{layout:?}");
        assert!(debug.contains("Grid"));
        assert!(debug.contains("2"));
    }

    #[test]
    fn test_clone_eq() {
        let a = TierLayout::Grid { columns: 4 };
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn test_serde_auto_round_trip() {
        let layout = TierLayout::Auto;
        let json = serde_json::to_string(&layout).unwrap();
        assert_eq!(json, "\"auto\"");
        let deserialized: TierLayout = serde_json::from_str(&json).unwrap();
        assert_eq!(layout, deserialized);
    }

    #[test]
    fn test_yaml_empty_map_rejected() {
        let yaml = "{}";
        let result: Result<TierLayout, _> = serde_yml::from_str(yaml);
        assert!(result.is_err());
    }
}
