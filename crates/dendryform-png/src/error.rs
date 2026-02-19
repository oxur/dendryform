//! PNG render error types.

use std::fmt;

/// Errors that occur during PNG rendering.
#[derive(Debug)]
#[non_exhaustive]
pub enum PngError {
    /// SVG rendering failed upstream.
    Svg(dendryform_svg::SvgError),
    /// resvg failed to parse the SVG tree.
    Parse(String),
    /// Failed to allocate the pixel buffer (dimensions too large or zero).
    Pixmap,
    /// PNG encoding failed.
    Encode(String),
}

impl fmt::Display for PngError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Svg(e) => write!(f, "SVG render error: {e}"),
            Self::Parse(e) => write!(f, "SVG parse error: {e}"),
            Self::Pixmap => write!(f, "failed to allocate pixel buffer (invalid dimensions)"),
            Self::Encode(e) => write!(f, "PNG encoding error: {e}"),
        }
    }
}

impl std::error::Error for PngError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Svg(e) => Some(e),
            _ => None,
        }
    }
}

impl From<dendryform_svg::SvgError> for PngError {
    fn from(e: dendryform_svg::SvgError) -> Self {
        Self::Svg(e)
    }
}
