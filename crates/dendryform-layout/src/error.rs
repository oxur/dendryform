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

#[cfg(test)]
mod tests {
    use super::*;
    use dendryform_core::ValidationError;

    #[test]
    fn test_layout_error_display_validation() {
        let inner = ValidationError::MissingField { field: "id" };
        let err = LayoutError::Validation(inner);
        let msg = format!("{err}");
        assert!(msg.contains("validation error during layout"));
        assert!(msg.contains("id"));
    }

    #[test]
    fn test_layout_error_display_constraint_violation() {
        let err = LayoutError::ConstraintViolation {
            message: "columns must be > 0".to_owned(),
        };
        let msg = format!("{err}");
        assert!(msg.contains("layout constraint"));
        assert!(msg.contains("columns must be > 0"));
    }

    #[test]
    fn test_layout_error_debug() {
        let err = LayoutError::ConstraintViolation {
            message: "test".to_owned(),
        };
        let debug = format!("{err:?}");
        assert!(debug.contains("ConstraintViolation"));
    }

    #[test]
    fn test_layout_error_source_validation() {
        let inner = ValidationError::MissingField { field: "id" };
        let err = LayoutError::Validation(inner);
        let source = std::error::Error::source(&err);
        assert!(source.is_some());
    }

    #[test]
    fn test_layout_error_source_constraint_is_none() {
        let err = LayoutError::ConstraintViolation {
            message: "test".to_owned(),
        };
        let source = std::error::Error::source(&err);
        assert!(source.is_none());
    }

    #[test]
    fn test_layout_error_from_validation_error() {
        let inner = ValidationError::EmptyTier {
            id: "t1".to_owned(),
        };
        let err: LayoutError = inner.into();
        assert!(matches!(err, LayoutError::Validation(_)));
    }
}
