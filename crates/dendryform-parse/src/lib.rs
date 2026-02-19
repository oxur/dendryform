//! # dendryform-parse
//!
//! YAML and JSON parser for dendryform diagram definitions.
//!
//! Reads a diagram file (YAML or JSON), deserializes it into a
//! `RawDiagram`, and validates it into a `Diagram` via
//! `Diagram::try_from`.

/// dendryform-parse is coming soon.
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
