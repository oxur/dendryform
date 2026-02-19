//! Visual connector types between tiers.

use std::fmt;

use serde::{Deserialize, Serialize};

/// The visual style of a connector between tiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ConnectorStyle {
    /// Solid vertical line with a downward arrowhead.
    Line,
    /// A row of small dots.
    Dots,
}

impl fmt::Display for ConnectorStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Line => f.write_str("line"),
            Self::Dots => f.write_str("dots"),
        }
    }
}

/// A visual connector between adjacent tiers.
///
/// Connectors provide visual flow cues but do not carry semantic
/// relationship data (use [`Edge`](crate::Edge) for that).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Connector {
    style: ConnectorStyle,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

impl Connector {
    /// Creates a new connector with the given style.
    pub fn new(style: ConnectorStyle) -> Self {
        Self { style, label: None }
    }

    /// Creates a new connector with a label (line connectors only).
    pub fn with_label(style: ConnectorStyle, label: &str) -> Self {
        Self {
            style,
            label: Some(label.to_owned()),
        }
    }

    /// Returns the visual style.
    pub fn style(&self) -> ConnectorStyle {
        self.style
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
        let conn = Connector::new(ConnectorStyle::Dots);
        assert_eq!(conn.style(), ConnectorStyle::Dots);
        assert_eq!(conn.label(), None);
    }

    #[test]
    fn test_with_label() {
        let conn = Connector::with_label(ConnectorStyle::Line, "HTTPS");
        assert_eq!(conn.style(), ConnectorStyle::Line);
        assert_eq!(conn.label(), Some("HTTPS"));
    }

    #[test]
    fn test_serde_round_trip() {
        let conn = Connector::with_label(ConnectorStyle::Line, "gRPC");
        let json = serde_json::to_string(&conn).unwrap();
        let deserialized: Connector = serde_json::from_str(&json).unwrap();
        assert_eq!(conn, deserialized);
    }
}
