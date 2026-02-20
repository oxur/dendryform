//! Font database setup for PNG rendering.

use std::sync::Arc;

use resvg::usvg::fontdb;

/// Creates a font database loaded with system fonts and optionally bundled fonts.
///
/// When the `bundled-fonts` feature is enabled, JetBrains Mono and DM Sans
/// are embedded directly in the binary. Otherwise, the function loads system
/// fonts and warns to stderr if JetBrains Mono is not found.
pub fn create_fontdb() -> Arc<fontdb::Database> {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();

    #[cfg(feature = "bundled-fonts")]
    {
        db.load_font_data(include_bytes!("../assets/fonts/JetBrainsMono-Regular.ttf").to_vec());
        db.load_font_data(include_bytes!("../assets/fonts/JetBrainsMono-Medium.ttf").to_vec());
        db.load_font_data(include_bytes!("../assets/fonts/DMSans-Regular.ttf").to_vec());
        db.load_font_data(include_bytes!("../assets/fonts/DMSans-Medium.ttf").to_vec());
    }

    let has_jetbrains = db.faces().any(|face| {
        face.families
            .iter()
            .any(|(name, _)| name == "JetBrains Mono")
    });

    if !has_jetbrains {
        eprintln!(
            "warning: JetBrains Mono not found; PNG text may render with a fallback font.\n\
             Install JetBrains Mono from https://www.jetbrains.com/lp/mono/ or rebuild\n\
             with bundled fonts: cargo build -p dendryform-cli --features bundled-fonts"
        );
    }

    Arc::new(db)
}
