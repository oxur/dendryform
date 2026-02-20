//! Theme system — colors, fonts, spacing, and the built-in dark theme.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::color::Color;

/// A set of color values for a single named color in the palette.
///
/// Each color has a primary accent (hex), a dim background tint (CSS rgba),
/// and a hover border color (hex, typically a lighter or shifted variant).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ColorSet {
    /// Primary accent color (hex, e.g. `"#4fc3f7"`).
    accent: String,
    /// Dim background tint (CSS color string, e.g. `"rgba(79, 195, 247, 0.10)"`).
    dim: String,
    /// Hover border color (hex).
    hover_border: String,
}

impl ColorSet {
    /// Creates a new color set.
    pub fn new(accent: &str, dim: &str, hover_border: &str) -> Self {
        Self {
            accent: accent.to_owned(),
            dim: dim.to_owned(),
            hover_border: hover_border.to_owned(),
        }
    }

    /// Returns the primary accent color (hex).
    pub fn accent(&self) -> &str {
        &self.accent
    }

    /// Returns the dim background tint (CSS color string).
    pub fn dim(&self) -> &str {
        &self.dim
    }

    /// Returns the hover border color (hex).
    pub fn hover_border(&self) -> &str {
        &self.hover_border
    }
}

/// The color palette — maps each [`Color`] to its [`ColorSet`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ThemePalette(HashMap<Color, ColorSet>);

impl ThemePalette {
    /// Creates a new palette from a map.
    pub fn new(entries: HashMap<Color, ColorSet>) -> Self {
        Self(entries)
    }

    /// Returns the color set for a given color, if defined.
    pub fn get(&self, color: Color) -> Option<&ColorSet> {
        self.0.get(&color)
    }

    /// Inserts or replaces a color set.
    pub fn insert(&mut self, color: Color, set: ColorSet) {
        self.0.insert(color, set);
    }

    /// Returns an iterator over all palette entries.
    pub fn iter(&self) -> impl Iterator<Item = (&Color, &ColorSet)> {
        self.0.iter()
    }

    /// Merges another palette on top of this one (overrides win).
    pub fn merge(&mut self, overrides: ThemePalette) {
        for (color, set) in overrides.0 {
            self.0.insert(color, set);
        }
    }
}

/// Background color configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Backgrounds {
    /// Primary page background (hex).
    page: String,
    /// Card background (hex).
    card: String,
    /// Card hover background (hex).
    card_hover: String,
}

impl Backgrounds {
    /// Creates a new background configuration.
    pub fn new(page: &str, card: &str, card_hover: &str) -> Self {
        Self {
            page: page.to_owned(),
            card: card.to_owned(),
            card_hover: card_hover.to_owned(),
        }
    }

    /// Returns the page background color.
    pub fn page(&self) -> &str {
        &self.page
    }

    /// Returns the card background color.
    pub fn card(&self) -> &str {
        &self.card
    }

    /// Returns the card hover background color.
    pub fn card_hover(&self) -> &str {
        &self.card_hover
    }
}

/// Text color configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextColors {
    /// Primary text color (hex).
    primary: String,
    /// Dimmed text color (hex).
    dim: String,
    /// Bright/emphasized text color (hex).
    bright: String,
}

impl TextColors {
    /// Creates a new text color configuration.
    pub fn new(primary: &str, dim: &str, bright: &str) -> Self {
        Self {
            primary: primary.to_owned(),
            dim: dim.to_owned(),
            bright: bright.to_owned(),
        }
    }

    /// Returns the primary text color.
    pub fn primary(&self) -> &str {
        &self.primary
    }

    /// Returns the dimmed text color.
    pub fn dim(&self) -> &str {
        &self.dim
    }

    /// Returns the bright text color.
    pub fn bright(&self) -> &str {
        &self.bright
    }
}

/// Border color configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Borders {
    /// Default border color (hex).
    normal: String,
    /// Highlighted/accent border color (hex).
    highlight: String,
}

impl Borders {
    /// Creates a new border configuration.
    pub fn new(normal: &str, highlight: &str) -> Self {
        Self {
            normal: normal.to_owned(),
            highlight: highlight.to_owned(),
        }
    }

    /// Returns the normal border color.
    pub fn normal(&self) -> &str {
        &self.normal
    }

    /// Returns the highlight border color.
    pub fn highlight(&self) -> &str {
        &self.highlight
    }
}

/// Font configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Fonts {
    /// Display / heading font family (e.g. `"JetBrains Mono"`).
    display: String,
    /// Body / text font family (e.g. `"DM Sans"`).
    body: String,
}

impl Fonts {
    /// Creates a new font configuration.
    pub fn new(display: &str, body: &str) -> Self {
        Self {
            display: display.to_owned(),
            body: body.to_owned(),
        }
    }

    /// Returns the display font family.
    pub fn display(&self) -> &str {
        &self.display
    }

    /// Returns the body font family.
    pub fn body(&self) -> &str {
        &self.body
    }
}

/// Spacing and shape configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Spacing {
    /// Standard border radius in pixels.
    radius: u32,
    /// Small border radius in pixels.
    radius_sm: u32,
    /// Box shadow CSS value.
    shadow: String,
    /// Canvas max-width in pixels.
    canvas_width: u32,
}

impl Spacing {
    /// Creates a new spacing configuration.
    pub fn new(radius: u32, radius_sm: u32, shadow: &str, canvas_width: u32) -> Self {
        Self {
            radius,
            radius_sm,
            shadow: shadow.to_owned(),
            canvas_width,
        }
    }

    /// Returns the standard border radius.
    pub fn radius(&self) -> u32 {
        self.radius
    }

    /// Returns the small border radius.
    pub fn radius_sm(&self) -> u32 {
        self.radius_sm
    }

    /// Returns the box shadow CSS value.
    pub fn shadow(&self) -> &str {
        &self.shadow
    }

    /// Returns the canvas max-width in pixels.
    pub fn canvas_width(&self) -> u32 {
        self.canvas_width
    }
}

/// A complete theme for rendering diagrams.
///
/// Contains all visual configuration: color palette, background/text/border
/// colors, fonts, spacing, and animation settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Theme {
    /// Theme name (e.g. `"dark"`).
    name: String,
    /// Color palette mapping named colors to CSS values.
    palette: ThemePalette,
    /// Background colors.
    backgrounds: Backgrounds,
    /// Text colors.
    text: TextColors,
    /// Border colors.
    borders: Borders,
    /// Font families.
    fonts: Fonts,
    /// Spacing and shape values.
    spacing: Spacing,
    /// Whether to enable CSS animations.
    animate: bool,
}

impl Theme {
    /// Returns the theme name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the color palette.
    pub fn palette(&self) -> &ThemePalette {
        &self.palette
    }

    /// Returns the background colors.
    pub fn backgrounds(&self) -> &Backgrounds {
        &self.backgrounds
    }

    /// Returns the text colors.
    pub fn text(&self) -> &TextColors {
        &self.text
    }

    /// Returns the border colors.
    pub fn borders(&self) -> &Borders {
        &self.borders
    }

    /// Returns the font families.
    pub fn fonts(&self) -> &Fonts {
        &self.fonts
    }

    /// Returns the spacing configuration.
    pub fn spacing(&self) -> &Spacing {
        &self.spacing
    }

    /// Returns whether animations are enabled.
    pub fn animate(&self) -> bool {
        self.animate
    }

    /// Returns the built-in dark theme with exact Taproot color values.
    pub fn dark() -> Self {
        let mut palette = HashMap::new();
        palette.insert(
            Color::Blue,
            ColorSet::new("#4fc3f7", "rgba(79, 195, 247, 0.10)", "#4fc3f7"),
        );
        palette.insert(
            Color::Green,
            ColorSet::new("#3ddc84", "rgba(61, 220, 132, 0.12)", "#3ddc84"),
        );
        palette.insert(
            Color::Amber,
            ColorSet::new("#ffb74d", "rgba(255, 183, 77, 0.10)", "#ffb74d"),
        );
        palette.insert(
            Color::Purple,
            ColorSet::new("#b39ddb", "rgba(179, 157, 219, 0.10)", "#b39ddb"),
        );
        palette.insert(
            Color::Red,
            ColorSet::new("#ef5350", "rgba(239, 83, 80, 0.10)", "#ef5350"),
        );
        palette.insert(
            Color::Teal,
            ColorSet::new("#4dd0e1", "rgba(77, 208, 225, 0.10)", "#4dd0e1"),
        );

        Self {
            name: "dark".to_owned(),
            palette: ThemePalette::new(palette),
            backgrounds: Backgrounds::new("#0a0e14", "#111820", "#161e29"),
            text: TextColors::new("#c4cdd9", "#5a6a7a", "#e8edf3"),
            borders: Borders::new("#1e2a3a", "#2a3f5a"),
            fonts: Fonts::new("JetBrains Mono", "DM Sans"),
            spacing: Spacing::new(10, 6, "0 2px 20px rgba(0,0,0,0.3)", 1100),
            animate: true,
        }
    }

    /// Merges another theme on top of this one.
    ///
    /// Override values replace base values. Palette entries are merged
    /// individually (only overridden colors are replaced).
    pub fn merge(mut self, overrides: ThemeOverrides) -> Self {
        if let Some(palette) = overrides.palette {
            self.palette.merge(palette);
        }
        if let Some(backgrounds) = overrides.backgrounds {
            self.backgrounds = backgrounds;
        }
        if let Some(text) = overrides.text {
            self.text = text;
        }
        if let Some(borders) = overrides.borders {
            self.borders = borders;
        }
        if let Some(fonts) = overrides.fonts {
            self.fonts = fonts;
        }
        if let Some(spacing) = overrides.spacing {
            self.spacing = spacing;
        }
        if let Some(animate) = overrides.animate {
            self.animate = animate;
        }
        self
    }
}

/// Partial theme overrides for merging on top of a base theme.
///
/// All fields are optional — only specified fields override the base.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ThemeOverrides {
    /// Override palette entries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub palette: Option<ThemePalette>,
    /// Override background colors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backgrounds: Option<Backgrounds>,
    /// Override text colors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<TextColors>,
    /// Override border colors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub borders: Option<Borders>,
    /// Override font families.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fonts: Option<Fonts>,
    /// Override spacing configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    /// Override animation setting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animate: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dark_theme_name() {
        let theme = Theme::dark();
        assert_eq!(theme.name(), "dark");
    }

    #[test]
    fn test_dark_theme_palette_has_all_colors() {
        let theme = Theme::dark();
        assert!(theme.palette().get(Color::Blue).is_some());
        assert!(theme.palette().get(Color::Green).is_some());
        assert!(theme.palette().get(Color::Amber).is_some());
        assert!(theme.palette().get(Color::Purple).is_some());
        assert!(theme.palette().get(Color::Red).is_some());
        assert!(theme.palette().get(Color::Teal).is_some());
    }

    #[test]
    fn test_dark_theme_exact_blue_values() {
        let theme = Theme::dark();
        let blue = theme.palette().get(Color::Blue).unwrap();
        assert_eq!(blue.accent(), "#4fc3f7");
        assert_eq!(blue.dim(), "rgba(79, 195, 247, 0.10)");
    }

    #[test]
    fn test_dark_theme_exact_green_values() {
        let theme = Theme::dark();
        let green = theme.palette().get(Color::Green).unwrap();
        assert_eq!(green.accent(), "#3ddc84");
        assert_eq!(green.dim(), "rgba(61, 220, 132, 0.12)");
    }

    #[test]
    fn test_dark_theme_backgrounds() {
        let theme = Theme::dark();
        assert_eq!(theme.backgrounds().page(), "#0a0e14");
        assert_eq!(theme.backgrounds().card(), "#111820");
        assert_eq!(theme.backgrounds().card_hover(), "#161e29");
    }

    #[test]
    fn test_dark_theme_text_colors() {
        let theme = Theme::dark();
        assert_eq!(theme.text().primary(), "#c4cdd9");
        assert_eq!(theme.text().dim(), "#5a6a7a");
        assert_eq!(theme.text().bright(), "#e8edf3");
    }

    #[test]
    fn test_dark_theme_borders() {
        let theme = Theme::dark();
        assert_eq!(theme.borders().normal(), "#1e2a3a");
        assert_eq!(theme.borders().highlight(), "#2a3f5a");
    }

    #[test]
    fn test_dark_theme_fonts() {
        let theme = Theme::dark();
        assert_eq!(theme.fonts().display(), "JetBrains Mono");
        assert_eq!(theme.fonts().body(), "DM Sans");
    }

    #[test]
    fn test_dark_theme_spacing() {
        let theme = Theme::dark();
        assert_eq!(theme.spacing().radius(), 10);
        assert_eq!(theme.spacing().radius_sm(), 6);
        assert_eq!(theme.spacing().canvas_width(), 1100);
    }

    #[test]
    fn test_dark_theme_animate() {
        let theme = Theme::dark();
        assert!(theme.animate());
    }

    #[test]
    fn test_merge_overrides_palette() {
        let base = Theme::dark();
        let mut override_palette = HashMap::new();
        override_palette.insert(
            Color::Blue,
            ColorSet::new("#0000ff", "rgba(0,0,255,0.1)", "#0000ff"),
        );
        let overrides = ThemeOverrides {
            palette: Some(ThemePalette::new(override_palette)),
            ..Default::default()
        };
        let merged = base.merge(overrides);
        assert_eq!(
            merged.palette().get(Color::Blue).unwrap().accent(),
            "#0000ff"
        );
        // Green should be unchanged.
        assert_eq!(
            merged.palette().get(Color::Green).unwrap().accent(),
            "#3ddc84"
        );
    }

    #[test]
    fn test_merge_overrides_animate() {
        let base = Theme::dark();
        let overrides = ThemeOverrides {
            animate: Some(false),
            ..Default::default()
        };
        let merged = base.merge(overrides);
        assert!(!merged.animate());
        // Everything else unchanged.
        assert_eq!(merged.name(), "dark");
        assert_eq!(merged.backgrounds().page(), "#0a0e14");
    }

    #[test]
    fn test_merge_overrides_backgrounds() {
        let base = Theme::dark();
        let overrides = ThemeOverrides {
            backgrounds: Some(Backgrounds::new("#000000", "#111111", "#222222")),
            ..Default::default()
        };
        let merged = base.merge(overrides);
        assert_eq!(merged.backgrounds().page(), "#000000");
        assert_eq!(merged.backgrounds().card(), "#111111");
    }

    #[test]
    fn test_serde_round_trip_theme() {
        let theme = Theme::dark();
        let json = serde_json::to_string_pretty(&theme).unwrap();
        let deserialized: Theme = serde_json::from_str(&json).unwrap();
        assert_eq!(theme, deserialized);
    }

    #[test]
    fn test_serde_round_trip_overrides() {
        let overrides = ThemeOverrides {
            animate: Some(false),
            backgrounds: Some(Backgrounds::new("#000", "#111", "#222")),
            ..Default::default()
        };
        let json = serde_json::to_string_pretty(&overrides).unwrap();
        let deserialized: ThemeOverrides = serde_json::from_str(&json).unwrap();
        assert_eq!(overrides, deserialized);
    }

    #[test]
    fn test_serde_overrides_empty_fields_omitted() {
        let overrides = ThemeOverrides {
            animate: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_string(&overrides).unwrap();
        assert!(!json.contains("palette"));
        assert!(!json.contains("backgrounds"));
        assert!(json.contains("animate"));
    }

    #[test]
    fn test_yaml_round_trip_theme() {
        let theme = Theme::dark();
        let yaml = serde_yml::to_string(&theme).unwrap();
        let deserialized: Theme = serde_yml::from_str(&yaml).unwrap();
        assert_eq!(theme, deserialized);
    }

    #[test]
    fn test_colorset_accessors() {
        let cs = ColorSet::new("#abc", "rgba(0,0,0,0.5)", "#def");
        assert_eq!(cs.accent(), "#abc");
        assert_eq!(cs.dim(), "rgba(0,0,0,0.5)");
        assert_eq!(cs.hover_border(), "#def");
    }

    #[test]
    fn test_palette_insert_and_get() {
        let mut palette = ThemePalette::new(HashMap::new());
        assert!(palette.get(Color::Red).is_none());
        palette.insert(
            Color::Red,
            ColorSet::new("#f00", "rgba(255,0,0,0.1)", "#f00"),
        );
        assert_eq!(palette.get(Color::Red).unwrap().accent(), "#f00");
    }

    #[test]
    fn test_palette_merge() {
        let mut base = ThemePalette::new(HashMap::new());
        base.insert(Color::Red, ColorSet::new("#f00", "r", "#f00"));
        base.insert(Color::Blue, ColorSet::new("#00f", "b", "#00f"));

        let mut overrides = ThemePalette::new(HashMap::new());
        overrides.insert(Color::Red, ColorSet::new("#ff0000", "rr", "#ff0000"));

        base.merge(overrides);
        assert_eq!(base.get(Color::Red).unwrap().accent(), "#ff0000");
        assert_eq!(base.get(Color::Blue).unwrap().accent(), "#00f");
    }

    #[test]
    fn test_dark_yml_matches_dark_constructor() {
        let yaml = include_str!("../../../themes/dark.yml");
        let from_file: Theme = serde_yml::from_str(yaml).unwrap();
        let from_code = Theme::dark();
        assert_eq!(from_file, from_code);
    }

    #[test]
    fn test_merge_overrides_text() {
        let base = Theme::dark();
        let overrides = ThemeOverrides {
            text: Some(TextColors::new("#aaa", "#bbb", "#ccc")),
            ..Default::default()
        };
        let merged = base.merge(overrides);
        assert_eq!(merged.text().primary(), "#aaa");
        assert_eq!(merged.text().dim(), "#bbb");
        assert_eq!(merged.text().bright(), "#ccc");
    }

    #[test]
    fn test_merge_overrides_borders() {
        let base = Theme::dark();
        let overrides = ThemeOverrides {
            borders: Some(Borders::new("#111", "#222")),
            ..Default::default()
        };
        let merged = base.merge(overrides);
        assert_eq!(merged.borders().normal(), "#111");
        assert_eq!(merged.borders().highlight(), "#222");
    }

    #[test]
    fn test_merge_overrides_fonts() {
        let base = Theme::dark();
        let overrides = ThemeOverrides {
            fonts: Some(Fonts::new("Fira Code", "Inter")),
            ..Default::default()
        };
        let merged = base.merge(overrides);
        assert_eq!(merged.fonts().display(), "Fira Code");
        assert_eq!(merged.fonts().body(), "Inter");
    }

    #[test]
    fn test_merge_overrides_spacing() {
        let base = Theme::dark();
        let overrides = ThemeOverrides {
            spacing: Some(Spacing::new(12, 8, "none", 900)),
            ..Default::default()
        };
        let merged = base.merge(overrides);
        assert_eq!(merged.spacing().radius(), 12);
        assert_eq!(merged.spacing().radius_sm(), 8);
        assert_eq!(merged.spacing().shadow(), "none");
        assert_eq!(merged.spacing().canvas_width(), 900);
    }

    #[test]
    fn test_palette_iter() {
        let theme = Theme::dark();
        let count = theme.palette().iter().count();
        assert_eq!(count, 6); // Blue, Green, Amber, Purple, Red, Teal
    }

    #[test]
    fn test_dark_theme_exact_amber_values() {
        let theme = Theme::dark();
        let amber = theme.palette().get(Color::Amber).unwrap();
        assert_eq!(amber.accent(), "#ffb74d");
        assert_eq!(amber.dim(), "rgba(255, 183, 77, 0.10)");
        assert_eq!(amber.hover_border(), "#ffb74d");
    }

    #[test]
    fn test_dark_theme_exact_purple_values() {
        let theme = Theme::dark();
        let purple = theme.palette().get(Color::Purple).unwrap();
        assert_eq!(purple.accent(), "#b39ddb");
    }

    #[test]
    fn test_dark_theme_exact_red_values() {
        let theme = Theme::dark();
        let red = theme.palette().get(Color::Red).unwrap();
        assert_eq!(red.accent(), "#ef5350");
    }

    #[test]
    fn test_dark_theme_exact_teal_values() {
        let theme = Theme::dark();
        let teal = theme.palette().get(Color::Teal).unwrap();
        assert_eq!(teal.accent(), "#4dd0e1");
    }
}
