//! Legend entry type.

use serde::{Deserialize, Serialize};

use crate::color::Color;

/// A single entry in the diagram legend.
///
/// Rendered as a colored swatch with a label.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LegendEntry {
    color: Color,
    label: String,
}

impl LegendEntry {
    /// Creates a new legend entry.
    pub fn new(color: Color, label: &str) -> Self {
        Self {
            color,
            label: label.to_owned(),
        }
    }

    /// Returns the swatch color.
    pub fn color(&self) -> Color {
        self.color
    }

    /// Returns the label text.
    pub fn label(&self) -> &str {
        &self.label
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let entry = LegendEntry::new(Color::Blue, "Clients");
        assert_eq!(entry.color(), Color::Blue);
        assert_eq!(entry.label(), "Clients");
    }

    #[test]
    fn test_serde_round_trip() {
        let entry = LegendEntry::new(Color::Purple, "Business Logic");
        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: LegendEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry, deserialized);
    }
}
