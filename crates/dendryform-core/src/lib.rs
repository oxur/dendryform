//! # dendryform-core
//!
//! Core schema types, validation, theme, and layout plan for dendryform.
//!
//! This crate defines the data model for describing software architecture
//! diagrams: nodes, edges, tiers, connectors, containers, and the theme
//! system. All types use private fields with validated constructors to
//! ensure invalid states are unrepresentable.
//!
//! ## Key Types
//!
//! - [`NodeId`] — validated slug identifier for nodes
//! - [`Node`] — a card in the diagram, built via [`NodeBuilder`]
//! - [`Edge`] — a semantic relationship between nodes
//! - [`Tier`] — a horizontal band of nodes
//! - [`Diagram`] — the validated top-level document
//!
//! ## Construction Patterns
//!
//! Types are constructed either programmatically (via builders and
//! constructors) or by deserializing YAML/JSON. Deserialization of
//! [`Diagram`] validates cross-cutting invariants automatically via
//! `#[serde(try_from)]`.

mod color;
mod connector;
mod container;
mod diagram;
mod edge;
mod error;
mod id;
mod kind;
mod layer;
mod layout;
mod legend;
mod metadata;
mod node;
mod tech;
mod theme;
mod tier;

// Re-export all public types at crate root for ergonomic imports.
pub use color::Color;
pub use connector::{Connector, ConnectorStyle};
pub use container::{Container, ContainerBorder};
pub use diagram::{Diagram, DiagramHeader, RawDiagram, Title};
pub use edge::Edge;
pub use error::ValidationError;
pub use id::NodeId;
pub use kind::{EdgeKind, NodeKind};
pub use layer::{FlowLabels, Layer};
pub use layout::TierLayout;
pub use legend::LegendEntry;
pub use metadata::Metadata;
pub use node::{Node, NodeBuilder};
pub use tech::Tech;
pub use theme::{
    Backgrounds, Borders, ColorSet, Fonts, Spacing, TextColors, Theme, ThemeOverrides, ThemePalette,
};
pub use tier::Tier;

/// Returns the version of the dendryform-core crate.
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
