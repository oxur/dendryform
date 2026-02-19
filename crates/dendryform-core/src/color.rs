//! Diagram color palette.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Named colors in the dendryform palette.
///
/// Each color provides an accent, a dim background tint, and a hover border.
/// The `Custom` variant allows extending beyond the built-in palette.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Color {
    /// Blue — typically used for clients and external users.
    Blue,
    /// Green — typically used for API layers and server components.
    Green,
    /// Amber — typically used for warnings or highlighted components.
    Amber,
    /// Purple — typically used for business logic and core services.
    Purple,
    /// Red — typically used for databases and critical infrastructure.
    Red,
    /// Teal — typically used for caches and auxiliary services.
    Teal,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blue => f.write_str("blue"),
            Self::Green => f.write_str("green"),
            Self::Amber => f.write_str("amber"),
            Self::Purple => f.write_str("purple"),
            Self::Red => f.write_str("red"),
            Self::Teal => f.write_str("teal"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Color::Blue), "blue");
        assert_eq!(format!("{}", Color::Amber), "amber");
    }

    #[test]
    fn test_serde_round_trip() {
        let color = Color::Purple;
        let json = serde_json::to_string(&color).unwrap();
        assert_eq!(json, "\"purple\"");
        let deserialized: Color = serde_json::from_str(&json).unwrap();
        assert_eq!(color, deserialized);
    }

    #[test]
    fn test_copy() {
        let a = Color::Red;
        let b = a;
        assert_eq!(a, b);
    }
}
