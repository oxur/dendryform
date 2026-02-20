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

#[cfg(test)]
mod tests {
    use super::*;
    use dendryform_core::ValidationError;

    #[test]
    fn test_display_yaml() {
        let yaml_err: Result<serde_yml::Value, _> = serde_yml::from_str("[invalid yaml {");
        let err = ParseError::Yaml(yaml_err.unwrap_err());
        let msg = format!("{err}");
        assert!(msg.contains("YAML parse error"));
    }

    #[test]
    fn test_display_json() {
        let json_err: Result<serde_json::Value, _> = serde_json::from_str("{invalid json");
        let err = ParseError::Json(json_err.unwrap_err());
        let msg = format!("{err}");
        assert!(msg.contains("JSON parse error"));
    }

    #[test]
    fn test_display_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = ParseError::Io(io_err);
        let msg = format!("{err}");
        assert!(msg.contains("I/O error"));
    }

    #[test]
    fn test_display_validation() {
        let inner = ValidationError::MissingField { field: "title" };
        let err = ParseError::Validation(inner);
        let msg = format!("{err}");
        assert!(msg.contains("validation error"));
        assert!(msg.contains("title"));
    }

    #[test]
    fn test_source_yaml() {
        let yaml_err: Result<serde_yml::Value, _> = serde_yml::from_str("[bad {");
        let err = ParseError::Yaml(yaml_err.unwrap_err());
        assert!(std::error::Error::source(&err).is_some());
    }

    #[test]
    fn test_source_json() {
        let json_err: Result<serde_json::Value, _> = serde_json::from_str("{bad");
        let err = ParseError::Json(json_err.unwrap_err());
        assert!(std::error::Error::source(&err).is_some());
    }

    #[test]
    fn test_source_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "test");
        let err = ParseError::Io(io_err);
        assert!(std::error::Error::source(&err).is_some());
    }

    #[test]
    fn test_source_validation() {
        let inner = ValidationError::MissingField { field: "id" };
        let err = ParseError::Validation(inner);
        assert!(std::error::Error::source(&err).is_some());
    }

    #[test]
    fn test_from_yaml_error() {
        let yaml_err: Result<serde_yml::Value, _> = serde_yml::from_str("[bad {");
        let inner = yaml_err.unwrap_err();
        let err: ParseError = inner.into();
        assert!(matches!(err, ParseError::Yaml(_)));
    }

    #[test]
    fn test_from_json_error() {
        let json_err: Result<serde_json::Value, _> = serde_json::from_str("{bad");
        let inner = json_err.unwrap_err();
        let err: ParseError = inner.into();
        assert!(matches!(err, ParseError::Json(_)));
    }

    #[test]
    fn test_from_io_error() {
        let inner = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let err: ParseError = inner.into();
        assert!(matches!(err, ParseError::Io(_)));
    }

    #[test]
    fn test_from_validation_error() {
        let inner = ValidationError::EmptyTier { id: "t".to_owned() };
        let err: ParseError = inner.into();
        assert!(matches!(err, ParseError::Validation(_)));
    }

    #[test]
    fn test_debug_format() {
        let err = ParseError::Validation(ValidationError::MissingField { field: "id" });
        let debug = format!("{err:?}");
        assert!(debug.contains("Validation"));
    }
}
