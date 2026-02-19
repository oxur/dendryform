//! # dendryform-svg
//!
//! Static SVG renderer for dendryform diagrams.
//!
//! Consumes a `LayoutPlan` and produces pixel-perfect SVG with
//! absolute coordinates, embedded fonts, and the dark Taproot theme.

/// dendryform-svg is coming soon.
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
