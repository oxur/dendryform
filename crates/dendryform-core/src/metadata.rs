//! Extensibility metadata type.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// An extensibility escape hatch for user-defined key-value pairs.
///
/// Wraps a `HashMap<String, String>` with explicit accessor methods.
/// Does **not** implement `Deref<Target = HashMap>` (AP-15).
///
/// # Examples
///
/// ```
/// use dendryform_core::Metadata;
///
/// let mut meta = Metadata::new();
/// meta.insert("owner", "platform-team");
/// assert_eq!(meta.get("owner"), Some("platform-team"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Metadata(HashMap<String, String>);

impl Metadata {
    /// Creates an empty metadata map.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Returns the value for the given key, if present.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|v| v.as_str())
    }

    /// Inserts a key-value pair, returning the previous value if any.
    pub fn insert(&mut self, key: &str, value: &str) -> Option<String> {
        self.0.insert(key.to_owned(), value.to_owned())
    }

    /// Returns an iterator over key-value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.0.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    /// Returns `true` if the metadata map is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_empty() {
        let meta = Metadata::new();
        assert!(meta.is_empty());
        assert_eq!(meta.len(), 0);
    }

    #[test]
    fn test_insert_and_get() {
        let mut meta = Metadata::new();
        meta.insert("team", "platform");
        assert_eq!(meta.get("team"), Some("platform"));
        assert_eq!(meta.get("missing"), None);
        assert!(!meta.is_empty());
        assert_eq!(meta.len(), 1);
    }

    #[test]
    fn test_serde_round_trip() {
        let mut meta = Metadata::new();
        meta.insert("owner", "alice");
        meta.insert("cost-center", "eng");

        let json = serde_json::to_string(&meta).unwrap();
        let deserialized: Metadata = serde_json::from_str(&json).unwrap();
        assert_eq!(meta, deserialized);
    }
}
