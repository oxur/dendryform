//! Render error types.

use std::fmt;

/// Errors that occur during HTML rendering.
#[derive(Debug)]
#[non_exhaustive]
pub enum RenderError {
    /// A formatting/writing error.
    Fmt(fmt::Error),
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fmt(e) => write!(f, "render formatting error: {e}"),
        }
    }
}

impl std::error::Error for RenderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Fmt(e) => Some(e),
        }
    }
}

impl From<fmt::Error> for RenderError {
    fn from(e: fmt::Error) -> Self {
        Self::Fmt(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_error_display_fmt() {
        let inner = fmt::Error;
        let err = RenderError::Fmt(inner);
        let msg = format!("{err}");
        assert!(msg.contains("render formatting error"));
    }

    #[test]
    fn test_render_error_debug() {
        let err = RenderError::Fmt(fmt::Error);
        let debug = format!("{err:?}");
        assert!(debug.contains("Fmt"));
    }

    #[test]
    fn test_render_error_source() {
        let err = RenderError::Fmt(fmt::Error);
        let source = std::error::Error::source(&err);
        assert!(source.is_some());
    }

    #[test]
    fn test_render_error_from_fmt_error() {
        let inner = fmt::Error;
        let err: RenderError = inner.into();
        assert!(matches!(err, RenderError::Fmt(_)));
    }
}
