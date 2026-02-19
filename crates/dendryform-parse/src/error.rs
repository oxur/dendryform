//! Parse error types.

use std::fmt;

/// Errors that occur when parsing a diagram file.
#[derive(Debug)]
#[non_exhaustive]
pub enum ParseError {
    /// YAML deserialization failed.
    Yaml(serde_yml::Error),
    /// JSON deserialization failed.
    Json(serde_json::Error),
    /// File I/O failed.
    Io(std::io::Error),
    /// Post-deserialization validation failed.
    Validation(dendryform_core::ValidationError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yaml(e) => write!(f, "YAML parse error: {e}"),
            Self::Json(e) => write!(f, "JSON parse error: {e}"),
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Validation(e) => write!(f, "validation error: {e}"),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Yaml(e) => Some(e),
            Self::Json(e) => Some(e),
            Self::Io(e) => Some(e),
            Self::Validation(e) => Some(e),
        }
    }
}

impl From<serde_yml::Error> for ParseError {
    fn from(e: serde_yml::Error) -> Self {
        Self::Yaml(e)
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

impl From<std::io::Error> for ParseError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<dendryform_core::ValidationError> for ParseError {
    fn from(e: dendryform_core::ValidationError) -> Self {
        Self::Validation(e)
    }
}
