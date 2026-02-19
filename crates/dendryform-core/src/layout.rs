//! Tier layout configuration.

use serde::{Deserialize, Serialize};

/// How nodes within a tier are arranged.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
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
}
