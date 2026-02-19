//! # dendryform-ascii
//!
//! ASCII art renderer for dendryform diagrams.
//!
//! Consumes a `LayoutPlan` and produces a text-based representation
//! suitable for terminals, READMEs, and documentation.

/// dendryform-ascii is coming soon.
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
