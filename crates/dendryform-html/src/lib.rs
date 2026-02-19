//! # dendryform-html
//!
//! Responsive HTML renderer for dendryform diagrams.
//!
//! Consumes a `LayoutPlan` and produces a self-contained HTML file
//! with embedded CSS, JavaScript, and fonts. The output is dark-themed
//! and interactive.

/// dendryform-html is coming soon.
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
