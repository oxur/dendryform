//! # dendryform-core
//!
//! Core schema types, validation, theme, and layout plan for dendryform.
//!
//! This crate defines the data model for describing software architecture
//! diagrams: nodes, edges, tiers, connectors, containers, and the theme
//! system. All types use private fields with validated constructors to
//! ensure invalid states are unrepresentable.

/// dendryform-core is coming soon.
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
