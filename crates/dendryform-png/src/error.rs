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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_png_error_display_svg() {
        let inner = dendryform_svg::SvgError::Fmt(fmt::Error);
        let err = PngError::Svg(inner);
        let msg = format!("{err}");
        assert!(msg.contains("SVG render error"));
    }

    #[test]
    fn test_png_error_display_parse() {
        let err = PngError::Parse("bad tree".to_owned());
        let msg = format!("{err}");
        assert!(msg.contains("SVG parse error"));
        assert!(msg.contains("bad tree"));
    }

    #[test]
    fn test_png_error_display_pixmap() {
        let err = PngError::Pixmap;
        let msg = format!("{err}");
        assert!(msg.contains("failed to allocate pixel buffer"));
    }

    #[test]
    fn test_png_error_display_encode() {
        let err = PngError::Encode("encoding failed".to_owned());
        let msg = format!("{err}");
        assert!(msg.contains("PNG encoding error"));
        assert!(msg.contains("encoding failed"));
    }

    #[test]
    fn test_png_error_debug() {
        let err = PngError::Pixmap;
        let debug = format!("{err:?}");
        assert!(debug.contains("Pixmap"));
    }

    #[test]
    fn test_png_error_source_svg() {
        let inner = dendryform_svg::SvgError::Fmt(fmt::Error);
        let err = PngError::Svg(inner);
        let source = std::error::Error::source(&err);
        assert!(source.is_some());
    }

    #[test]
    fn test_png_error_source_parse_is_none() {
        let err = PngError::Parse("test".to_owned());
        let source = std::error::Error::source(&err);
        assert!(source.is_none());
    }

    #[test]
    fn test_png_error_source_pixmap_is_none() {
        let err = PngError::Pixmap;
        let source = std::error::Error::source(&err);
        assert!(source.is_none());
    }

    #[test]
    fn test_png_error_source_encode_is_none() {
        let err = PngError::Encode("test".to_owned());
        let source = std::error::Error::source(&err);
        assert!(source.is_none());
    }

    #[test]
    fn test_png_error_from_svg_error() {
        let inner = dendryform_svg::SvgError::Fmt(fmt::Error);
        let err: PngError = inner.into();
        assert!(matches!(err, PngError::Svg(_)));
    }
}
