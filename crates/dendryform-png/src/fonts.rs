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
            "warning: JetBrains Mono not found. \
             Install it or enable the 'bundled-fonts' feature for best results."
        );
    }

    Arc::new(db)
}
