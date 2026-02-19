//! Layout error types.

use std::fmt;

/// Errors that occur during layout computation.
#[derive(Debug)]
#[non_exhaustive]
pub enum LayoutError {
    /// A core validation error propagated during layout.
    Validation(dendryform_core::ValidationError),
    /// A layout constraint could not be satisfied.
    ConstraintViolation {
        /// Description of the violated constraint.
        message: String,
    },
}

impl fmt::Display for LayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error during layout: {e}"),
            Self::ConstraintViolation { message } => write!(f, "layout constraint: {message}"),
        }
    }
}

impl std::error::Error for LayoutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::ConstraintViolation { .. } => None,
        }
    }
}

impl From<dendryform_core::ValidationError> for LayoutError {
    fn from(e: dendryform_core::ValidationError) -> Self {
        Self::Validation(e)
    }
}
