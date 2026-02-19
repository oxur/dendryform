//! # dendryform-layout
//!
//! Shared layout engine that produces a `LayoutPlan` from a `Diagram`.
//!
//! The layout plan contains relative positioning, geometry, and
//! connector routing. Format-specific renderers (HTML, SVG) consume
//! this plan to produce output in their native coordinate systems.

/// dendryform-layout is coming soon.
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
