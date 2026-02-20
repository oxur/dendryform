//! Technology badge type.

use std::fmt;

use serde::{Deserialize, Serialize};

/// A technology badge displayed on a node card.
///
/// Wraps a non-empty string. Rendered as a pill/badge in the diagram.
///
/// # Examples
///
/// ```
/// use dendryform_core::Tech;
///
/// let tech = Tech::new("React");
/// assert_eq!(tech.as_str(), "React");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Tech(String);

impl Tech {
    /// Creates a new technology badge.
    pub fn new(value: &str) -> Self {
        Self(value.to_owned())
    }

    /// Returns the technology name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Tech {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let tech = Tech::new("TypeScript");
        assert_eq!(format!("{tech}"), "TypeScript");
    }

    #[test]
    fn test_serde_round_trip() {
        let tech = Tech::new("axum");
        let json = serde_json::to_string(&tech).unwrap();
        assert_eq!(json, "\"axum\"");
        let deserialized: Tech = serde_json::from_str(&json).unwrap();
        assert_eq!(tech, deserialized);
    }

    #[test]
    fn test_as_str() {
        let tech = Tech::new("React");
        assert_eq!(tech.as_str(), "React");
    }

    #[test]
    fn test_debug() {
        let tech = Tech::new("Rust");
        let debug = format!("{tech:?}");
        assert!(debug.contains("Rust"));
    }

    #[test]
    fn test_clone_eq() {
        let a = Tech::new("Go");
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Tech::new("Rust"));
        set.insert(Tech::new("Go"));
        set.insert(Tech::new("Rust"));
        assert_eq!(set.len(), 2);
    }
}
