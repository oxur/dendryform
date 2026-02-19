//! # dendryform-png
//!
//! PNG renderer for dendryform diagrams.
//!
//! Wraps `dendryform-svg` output and rasterizes it to PNG using
//! `resvg`. This is a thin adapter — all layout and visual logic
//! lives in the SVG crate.

/// dendryform-png is coming soon.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_is_set() {
        assert_eq!(version(), "0.0.1");
    }
}
