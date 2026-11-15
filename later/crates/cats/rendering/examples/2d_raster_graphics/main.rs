//! Runs the headless 2D raster graphics example.
//!
//! TODO Reviewed ??-??-2026.

/// Draw shapes into a pixel buffer with `tiny-skia`.
mod tiny_skia;

/// Runs the 2D raster graphics example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tiny_skia::run()
}
