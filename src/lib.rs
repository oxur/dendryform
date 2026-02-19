//! # dendryform
//!
//! Declarative software architecture diagrams — beautiful, dark-themed HTML
//! from a simple schema.
//!
//! Named for the 23 dendriform models of tree architecture
//! (Hallé & Oldeman, 1970), because every system has a branching pattern
//! worth revealing.
//!
//! ## Status
//!
//! This crate is under active development. The API is not yet stable.
//!
//! ## Overview
//!
//! `dendryform` takes a declarative description of a software system — nodes,
//! edges, containment, tiers — and renders it as a beautiful, interactive HTML
//! architecture diagram.
//!
//! Think of it as: **Structurizr's data model** meets **hand-crafted visual
//! design** meets **Rust's type safety**.

/// dendryform is coming soon.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        assert_eq!(version(), "0.0.1");
    }
}
