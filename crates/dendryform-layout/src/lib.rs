//! # dendryform-layout
//!
//! Shared layout engine that produces a [`LayoutPlan`] from a [`Diagram`](dendryform_core::Diagram).
//!
//! The layout plan contains relative positioning, geometry, and
//! connector routing. Format-specific renderers (HTML, SVG) consume
//! this plan to produce output in their native coordinate systems.
//!
//! ## Quick Start
//!
//! ```no_run
//! use dendryform_layout::compute_layout;
//!
//! let diagram = dendryform_parse::parse_yaml_file("examples/taproot/taproot.yml").unwrap();
//! let plan = compute_layout(&diagram).unwrap();
//! println!("Layers: {}", plan.layers.len());
//! ```

mod compute;
mod error;
mod geometry;

pub use compute::compute_layout;
pub use error::LayoutError;
pub use geometry::{
    ConnectorGeometry, ContainerGeometry, FlowLabelsGeometry, HeaderGeometry, LayerGeometry,
    LayoutPlan, LegendGeometry, NodeGeometry, TierGeometry, ViewportHint,
};

/// Returns the version of the dendryform-layout crate.
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
