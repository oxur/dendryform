//! # dendryform-export
//!
//! Lossy exporters for dendryform diagrams.
//!
//! Exports dendryform diagrams to standards-compatible formats:
//! Structurizr DSL, Structurizr JSON, and Mermaid. These exports
//! are lossy — dendryform's richer model is mapped onto the target
//! format's capabilities, with `LossyWarning`s for anything that
//! cannot be represented.

/// dendryform-export is coming soon.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_is_set() {
        assert_eq!(version(), "0.1.0");
    }
}
