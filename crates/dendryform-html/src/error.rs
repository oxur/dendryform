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
