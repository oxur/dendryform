//! Node identity type with slug validation.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::error::ValidationError;

/// A validated node identifier.
///
/// Node IDs must be valid slugs: start with a lowercase letter or digit,
/// followed by lowercase alphanumeric characters, hyphens, underscores,
/// or dots. For example: `"web-app"`, `"auth.service"`, `"db-01"`.
///
/// # Examples
///
/// ```
/// use dendryform_core::NodeId;
///
/// let id = NodeId::new("web-app").unwrap();
/// assert_eq!(id.as_str(), "web-app");
///
/// assert!(NodeId::new("").is_err());
/// assert!(NodeId::new("Has Spaces").is_err());
/// assert!(NodeId::new("UPPERCASE").is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct NodeId(String);

impl NodeId {
    /// Creates a new `NodeId` after validating the slug format.
    ///
    /// Valid slugs match `[a-z0-9][a-z0-9._-]*`.
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        Self::validate(value)?;
        Ok(Self(value.to_owned()))
    }

    /// Returns the ID as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), ValidationError> {
        if value.is_empty() {
            return Err(ValidationError::InvalidNodeId {
                value: value.to_owned(),
                reason: "node ID cannot be empty",
            });
        }

        let first = value.as_bytes()[0];
        if !first.is_ascii_lowercase() && !first.is_ascii_digit() {
            return Err(ValidationError::InvalidNodeId {
                value: value.to_owned(),
                reason: "must start with a lowercase letter or digit",
            });
        }

        if let Some(pos) = value.bytes().position(|b| {
            !b.is_ascii_lowercase() && !b.is_ascii_digit() && b != b'-' && b != b'_' && b != b'.'
        }) {
            let _ = pos;
            return Err(ValidationError::InvalidNodeId {
                value: value.to_owned(),
                reason: "must contain only lowercase alphanumeric, hyphens, underscores, or dots",
            });
        }

        Ok(())
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for NodeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NodeId::new(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_node_ids() {
        assert!(NodeId::new("web-app").is_ok());
        assert!(NodeId::new("auth.service").is_ok());
        assert!(NodeId::new("db-01").is_ok());
        assert!(NodeId::new("a").is_ok());
        assert!(NodeId::new("1password").is_ok());
        assert!(NodeId::new("my_component").is_ok());
    }

    #[test]
    fn test_empty_rejected() {
        assert!(NodeId::new("").is_err());
    }

    #[test]
    fn test_uppercase_rejected() {
        assert!(NodeId::new("WebApp").is_err());
        assert!(NodeId::new("ALLCAPS").is_err());
    }

    #[test]
    fn test_spaces_rejected() {
        assert!(NodeId::new("web app").is_err());
        assert!(NodeId::new(" leading").is_err());
    }

    #[test]
    fn test_special_chars_rejected() {
        assert!(NodeId::new("web@app").is_err());
        assert!(NodeId::new("web/app").is_err());
        assert!(NodeId::new("web#app").is_err());
    }

    #[test]
    fn test_starts_with_hyphen_rejected() {
        assert!(NodeId::new("-web").is_err());
    }

    #[test]
    fn test_display() {
        let id = NodeId::new("web-app").unwrap();
        assert_eq!(format!("{id}"), "web-app");
    }

    #[test]
    fn test_serde_round_trip() {
        let id = NodeId::new("web-app").unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"web-app\"");
        let deserialized: NodeId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, deserialized);
    }

    #[test]
    fn test_serde_rejects_invalid() {
        let result: Result<NodeId, _> = serde_json::from_str("\"Has Spaces\"");
        assert!(result.is_err());
    }
}
