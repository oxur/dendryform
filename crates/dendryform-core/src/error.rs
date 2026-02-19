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
