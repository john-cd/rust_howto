// TODO Reviewed ??-??-2026.
//! Runnable entry point for the text rendering example.
/// Provides rasterize glyph behavior for the text rendering component.
mod rasterize_glyph;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    rasterize_glyph::run()
}
