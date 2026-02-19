//! Layout geometry types — the positioned output of layout computation.

use dendryform_core::{
    Color, ConnectorStyle, ContainerBorder, LegendEntry, Node, NodeId, TierLayout,
};

/// Preferred viewport configuration for rendering.
#[derive(Debug, Clone, PartialEq)]
pub struct ViewportHint {
    /// Reference width in pixels (default 1100).
    pub width: f32,
    /// Horizontal padding in pixels.
    pub padding_x: f32,
    /// Top padding in pixels.
    pub padding_top: f32,
    /// Bottom padding in pixels.
    pub padding_bottom: f32,
}

impl Default for ViewportHint {
    fn default() -> Self {
        Self {
            width: 1100.0,
            padding_x: 32.0,
            padding_top: 48.0,
            padding_bottom: 64.0,
        }
    }
}

/// Header geometry — title and subtitle positioning.
#[derive(Debug, Clone, PartialEq)]
pub struct HeaderGeometry {
    /// The main title text.
    pub title_text: String,
    /// The accent word in the title.
    pub title_accent: String,
    /// The subtitle text.
    pub subtitle: String,
}

/// A positioned node within a tier grid.
#[derive(Debug, Clone, PartialEq)]
pub struct NodeGeometry<'a> {
    /// Reference to the source node data.
    pub node: &'a Node,
    /// Grid column index (0-based).
    pub grid_column: usize,
    /// Grid row index (0-based).
    pub grid_row: usize,
}

/// Layout of a connector between tiers.
#[derive(Debug, Clone, PartialEq)]
pub struct ConnectorGeometry {
    /// Visual style (line or dots).
    pub style: ConnectorStyle,
    /// Optional protocol/description label.
    pub label: Option<String>,
    /// Whether this connector is inside a container.
    pub is_internal: bool,
}

/// Layout of flow labels between tiers.
#[derive(Debug, Clone, PartialEq)]
pub struct FlowLabelsGeometry {
    /// The label texts.
    pub items: Vec<String>,
}

/// Layout of a container wrapping nested tiers.
#[derive(Debug, Clone, PartialEq)]
pub struct ContainerGeometry<'a> {
    /// Container label text.
    pub label: String,
    /// Container border style.
    pub border: ContainerBorder,
    /// Container label color.
    pub label_color: Color,
    /// Nested layer geometries.
    pub layers: Vec<LayerGeometry<'a>>,
}

/// Layout of a tier (horizontal band of nodes).
#[derive(Debug, Clone, PartialEq)]
pub struct TierGeometry<'a> {
    /// Tier identifier.
    pub id: NodeId,
    /// Optional heading label.
    pub label: Option<String>,
    /// The layout mode for this tier.
    pub layout: TierLayout,
    /// Number of grid columns.
    pub columns: usize,
    /// Positioned nodes in grid order.
    pub nodes: Vec<NodeGeometry<'a>>,
    /// Optional container wrapping nested tiers.
    pub container: Option<ContainerGeometry<'a>>,
}

/// A single positioned layer in the layout.
#[derive(Debug, Clone, PartialEq)]
pub enum LayerGeometry<'a> {
    /// A positioned tier.
    Tier(TierGeometry<'a>),
    /// A positioned connector.
    Connector(ConnectorGeometry),
    /// Positioned flow labels.
    FlowLabels(FlowLabelsGeometry),
}

/// Layout of the legend.
#[derive(Debug, Clone, PartialEq)]
pub struct LegendGeometry {
    /// Legend entries with color swatches.
    pub entries: Vec<LegendEntry>,
}

/// The complete layout plan for a diagram.
///
/// Contains all the geometry needed for rendering. Format-specific renderers
/// (HTML, SVG) consume this plan to produce output.
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutPlan<'a> {
    /// Viewport configuration.
    pub viewport: ViewportHint,
    /// Header geometry.
    pub header: HeaderGeometry,
    /// Ordered layer geometries (tiers, connectors, flow labels).
    pub layers: Vec<LayerGeometry<'a>>,
    /// Legend geometry.
    pub legend: LegendGeometry,
}
