//! Error types for the dendryform-core crate.

use std::fmt;

/// Errors that occur when constructing or validating core types.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValidationError {
    /// A node ID contains invalid characters or is empty.
    InvalidNodeId {
        /// The invalid ID value that was rejected.
        value: String,
        /// What was wrong with it.
        reason: &'static str,
    },

    /// Two or more nodes share the same ID.
    DuplicateNodeId {
        /// The duplicated ID.
        id: String,
    },

    /// An edge references a node ID that does not exist in the diagram.
    DanglingEdgeReference {
        /// The edge's source or target that doesn't exist.
        id: String,
        /// Which field was invalid ("from" or "to").
        field: &'static str,
    },

    /// A tier has neither nodes nor a container.
    EmptyTier {
        /// The tier's ID.
        id: String,
    },

    /// Container nesting exceeds the maximum depth.
    NestingTooDeep {
        /// The maximum allowed depth.
        max_depth: usize,
        /// The actual depth found.
        actual_depth: usize,
    },

    /// A required field is missing.
    MissingField {
        /// The name of the missing field.
        field: &'static str,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeId { value, reason } => {
                write!(f, "invalid node ID {value:?}: {reason}")
            }
            Self::DuplicateNodeId { id } => {
                write!(f, "duplicate node ID: {id:?}")
            }
            Self::DanglingEdgeReference { id, field } => {
                write!(f, "edge {field} references unknown node: {id:?}")
            }
            Self::EmptyTier { id } => {
                write!(f, "tier {id:?} has neither nodes nor a container")
            }
            Self::NestingTooDeep {
                max_depth,
                actual_depth,
            } => {
                write!(
                    f,
                    "container nesting depth {actual_depth} exceeds maximum {max_depth}"
                )
            }
            Self::MissingField { field } => {
                write!(f, "missing required field: {field}")
            }
        }
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_invalid_node_id() {
        let err = ValidationError::InvalidNodeId {
            value: "Bad ID".to_owned(),
            reason: "contains spaces",
        };
        let msg = format!("{err}");
        assert!(msg.contains("invalid node ID"));
        assert!(msg.contains("Bad ID"));
        assert!(msg.contains("contains spaces"));
    }

    #[test]
    fn test_display_duplicate_node_id() {
        let err = ValidationError::DuplicateNodeId {
            id: "app".to_owned(),
        };
        let msg = format!("{err}");
        assert!(msg.contains("duplicate node ID"));
        assert!(msg.contains("app"));
    }

    #[test]
    fn test_display_dangling_edge_reference() {
        let err = ValidationError::DanglingEdgeReference {
            id: "ghost".to_owned(),
            field: "from",
        };
        let msg = format!("{err}");
        assert!(msg.contains("edge from references unknown node"));
        assert!(msg.contains("ghost"));
    }

    #[test]
    fn test_display_empty_tier() {
        let err = ValidationError::EmptyTier {
            id: "tier-1".to_owned(),
        };
        let msg = format!("{err}");
        assert!(msg.contains("tier"));
        assert!(msg.contains("tier-1"));
        assert!(msg.contains("neither nodes nor a container"));
    }

    #[test]
    fn test_display_nesting_too_deep() {
        let err = ValidationError::NestingTooDeep {
            max_depth: 3,
            actual_depth: 5,
        };
        let msg = format!("{err}");
        assert!(msg.contains("nesting depth 5"));
        assert!(msg.contains("maximum 3"));
    }

    #[test]
    fn test_display_missing_field() {
        let err = ValidationError::MissingField { field: "title" };
        let msg = format!("{err}");
        assert!(msg.contains("missing required field"));
        assert!(msg.contains("title"));
    }

    #[test]
    fn test_debug_format() {
        let err = ValidationError::MissingField { field: "id" };
        let debug = format!("{err:?}");
        assert!(debug.contains("MissingField"));
    }

    #[test]
    fn test_clone_and_eq() {
        let err = ValidationError::DuplicateNodeId {
            id: "app".to_owned(),
        };
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }

    #[test]
    fn test_error_trait_source_is_none() {
        let err = ValidationError::MissingField { field: "id" };
        let source = std::error::Error::source(&err);
        assert!(source.is_none());
    }
}
