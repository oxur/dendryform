//! SVG render error types.

use std::fmt;

/// Errors that occur during SVG rendering.
#[derive(Debug)]
#[non_exhaustive]
pub enum SvgError {
    /// A formatting/writing error.
    Fmt(fmt::Error),
}

impl fmt::Display for SvgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fmt(e) => write!(f, "SVG render formatting error: {e}"),
        }
    }
}

impl std::error::Error for SvgError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Fmt(e) => Some(e),
        }
    }
}

impl From<fmt::Error> for SvgError {
    fn from(e: fmt::Error) -> Self {
        Self::Fmt(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_error_display_fmt() {
        let inner = fmt::Error;
        let err = SvgError::Fmt(inner);
        let msg = format!("{err}");
        assert!(msg.contains("SVG render formatting error"));
    }

    #[test]
    fn test_svg_error_debug() {
        let err = SvgError::Fmt(fmt::Error);
        let debug = format!("{err:?}");
        assert!(debug.contains("Fmt"));
    }

    #[test]
    fn test_svg_error_source() {
        let err = SvgError::Fmt(fmt::Error);
        let source = std::error::Error::source(&err);
        assert!(source.is_some());
    }

    #[test]
    fn test_svg_error_from_fmt_error() {
        let inner = fmt::Error;
        let err: SvgError = inner.into();
        assert!(matches!(err, SvgError::Fmt(_)));
    }
}
