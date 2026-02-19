//! SVG spacing and sizing constants.
//!
//! All values are in pixels and derived from the CSS in the HTML renderer
//! to maintain visual consistency between outputs.

/// All spacing and sizing constants for the SVG renderer.
#[allow(dead_code)]
pub struct SvgMetrics {
    // -- Canvas --
    /// Horizontal canvas padding.
    pub padding_x: f32,
    /// Top canvas padding.
    pub padding_top: f32,
    /// Bottom canvas padding.
    pub padding_bottom: f32,

    // -- Header --
    /// Title font size.
    pub title_font_size: f32,
    /// Subtitle font size.
    pub subtitle_font_size: f32,
    /// Gap between title and subtitle.
    pub title_subtitle_gap: f32,
    /// Margin below the entire header section.
    pub header_margin_bottom: f32,

    // -- Tier label --
    /// Tier label font size.
    pub tier_label_font_size: f32,
    /// Tier label letter spacing.
    pub tier_label_letter_spacing: f32,
    /// Margin below tier label.
    pub tier_label_margin_bottom: f32,
    /// Margin below the tier section.
    pub tier_margin_bottom: f32,

    // -- Node --
    /// Node horizontal padding.
    pub node_padding_x: f32,
    /// Node vertical padding.
    pub node_padding_y: f32,
    /// Node title font size.
    pub node_title_font_size: f32,
    /// Node icon font size.
    pub node_icon_font_size: f32,
    /// Gap between icon and title text.
    pub node_icon_gap: f32,
    /// Node description font size.
    pub node_desc_font_size: f32,
    /// Gap between title and description.
    pub node_title_desc_gap: f32,
    /// Node tech badge font size.
    pub node_tech_font_size: f32,
    /// Margin above tech badges.
    pub node_tech_margin_top: f32,
    /// Gap between tech badges.
    pub node_tech_gap: f32,
    /// Tech badge horizontal padding.
    pub node_tech_pad_x: f32,
    /// Tech badge vertical padding.
    pub node_tech_pad_y: f32,
    /// Accent bar height at top of node.
    pub node_accent_bar_height: f32,
    /// Node border radius.
    pub node_border_radius: f32,
    /// Node border width.
    pub node_border_width: f32,

    // -- Grid --
    /// Gap between grid cells.
    pub grid_gap: f32,
    /// Margin below grid.
    pub grid_margin_bottom: f32,

    // -- Connector --
    /// Connector line height.
    pub connector_height: f32,
    /// Connector line width.
    pub connector_width: f32,
    /// Vertical margin around connector.
    pub connector_margin_y: f32,
    /// Arrowhead width (half-width of the triangle base).
    pub arrowhead_half_width: f32,
    /// Arrowhead height.
    pub arrowhead_height: f32,
    /// Protocol label font size.
    pub protocol_label_font_size: f32,
    /// Gap between connector line and protocol label.
    pub protocol_label_gap: f32,

    // -- Internal connector (dots) --
    /// Number of dots.
    pub dot_count: usize,
    /// Dot diameter.
    pub dot_diameter: f32,
    /// Gap between dots.
    pub dot_gap: f32,
    /// Vertical margin around dots.
    pub dot_margin_y: f32,

    // -- Container --
    /// Solid container padding.
    pub container_solid_padding: f32,
    /// Dashed container padding.
    pub container_dashed_padding: f32,
    /// Container border radius.
    pub container_border_radius: f32,
    /// Container label font size.
    pub container_label_font_size: f32,
    /// Container label letter spacing.
    pub container_label_letter_spacing: f32,
    /// Container label y offset from top border.
    pub container_label_y_offset: f32,
    /// Container margin below (for dashed).
    pub container_margin_bottom: f32,
    /// Dashed container label font size.
    pub container_dashed_label_font_size: f32,

    // -- Flow labels --
    /// Flow label font size.
    pub flow_label_font_size: f32,
    /// Gap between flow label items.
    pub flow_label_gap: f32,
    /// Vertical margin around flow labels.
    pub flow_label_margin_y: f32,
    /// Arrow font size in flow labels.
    pub flow_label_arrow_font_size: f32,

    // -- Legend --
    /// Legend swatch width.
    pub legend_swatch_size: f32,
    /// Legend swatch border radius.
    pub legend_swatch_radius: f32,
    /// Gap between swatch and label text.
    pub legend_swatch_text_gap: f32,
    /// Gap between legend items.
    pub legend_item_gap: f32,
    /// Legend font size.
    pub legend_font_size: f32,
    /// Margin above legend.
    pub legend_margin_top: f32,

    // -- Text metrics --
    /// Character width ratio for monospace fonts (char_width = font_size * ratio).
    pub mono_char_width_ratio: f32,
    /// Line height ratio for text.
    pub line_height_ratio: f32,
}

impl Default for SvgMetrics {
    fn default() -> Self {
        Self {
            // Canvas
            padding_x: 32.0,
            padding_top: 48.0,
            padding_bottom: 64.0,

            // Header
            title_font_size: 28.0,
            subtitle_font_size: 13.0,
            title_subtitle_gap: 8.0,
            header_margin_bottom: 56.0,

            // Tier label
            tier_label_font_size: 10.0,
            tier_label_letter_spacing: 2.0,
            tier_label_margin_bottom: 10.0,
            tier_margin_bottom: 20.0,

            // Node
            node_padding_x: 20.0,
            node_padding_y: 16.0,
            node_title_font_size: 14.0,
            node_icon_font_size: 15.0,
            node_icon_gap: 8.0,
            node_desc_font_size: 12.0,
            node_title_desc_gap: 4.0,
            node_tech_font_size: 10.0,
            node_tech_margin_top: 8.0,
            node_tech_gap: 6.0,
            node_tech_pad_x: 8.0,
            node_tech_pad_y: 2.0,
            node_accent_bar_height: 2.0,
            node_border_radius: 10.0,
            node_border_width: 1.0,

            // Grid
            grid_gap: 12.0,
            grid_margin_bottom: 16.0,

            // Connector
            connector_height: 28.0,
            connector_width: 2.0,
            connector_margin_y: 4.0,
            arrowhead_half_width: 4.0,
            arrowhead_height: 5.0,
            protocol_label_font_size: 10.0,
            protocol_label_gap: 16.0,

            // Dots
            dot_count: 5,
            dot_diameter: 3.0,
            dot_gap: 4.0,
            dot_margin_y: 10.0,

            // Container
            container_solid_padding: 24.0,
            container_dashed_padding: 18.0,
            container_border_radius: 14.0,
            container_label_font_size: 10.0,
            container_label_letter_spacing: 2.0,
            container_label_y_offset: 10.0,
            container_margin_bottom: 16.0,
            container_dashed_label_font_size: 9.0,

            // Flow labels
            flow_label_font_size: 9.0,
            flow_label_gap: 48.0,
            flow_label_margin_y: 8.0,
            flow_label_arrow_font_size: 9.0,

            // Legend
            legend_swatch_size: 10.0,
            legend_swatch_radius: 3.0,
            legend_swatch_text_gap: 6.0,
            legend_item_gap: 24.0,
            legend_font_size: 10.0,
            legend_margin_top: 40.0,

            // Text metrics
            mono_char_width_ratio: 0.6,
            line_height_ratio: 1.5,
        }
    }
}

impl SvgMetrics {
    /// Estimates the pixel width of monospace text at a given font size.
    pub fn mono_text_width(&self, text: &str, font_size: f32) -> f32 {
        text.len() as f32 * font_size * self.mono_char_width_ratio
    }

    /// Estimates the pixel width of proportional (body) text at a given font size.
    /// Uses a slightly narrower ratio than monospace.
    #[allow(dead_code)]
    pub fn body_text_width(&self, text: &str, font_size: f32) -> f32 {
        text.len() as f32 * font_size * 0.5
    }

    /// Returns the line height for a given font size.
    pub fn line_height(&self, font_size: f32) -> f32 {
        font_size * self.line_height_ratio
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_metrics() {
        let m = SvgMetrics::default();
        assert!((m.title_font_size - 28.0).abs() < f32::EPSILON);
        assert!((m.grid_gap - 12.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_mono_text_width() {
        let m = SvgMetrics::default();
        // "hello" = 5 chars at 14px * 0.6 = 42.0
        let width = m.mono_text_width("hello", 14.0);
        assert!((width - 42.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_body_text_width() {
        let m = SvgMetrics::default();
        // "hello" = 5 chars at 12px * 0.5 = 30.0
        let width = m.body_text_width("hello", 12.0);
        assert!((width - 30.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_line_height() {
        let m = SvgMetrics::default();
        let lh = m.line_height(14.0);
        assert!((lh - 21.0).abs() < f32::EPSILON);
    }
}
