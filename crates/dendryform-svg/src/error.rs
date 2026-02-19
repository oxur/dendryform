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
