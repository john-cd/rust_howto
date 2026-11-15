//! Runs the SVG parsing and raster-rendering examples.
//!
//! TODO Reviewed ??-??-2026.

/// Parses an SVG and rasterizes it to a PNG with `resvg`.
mod resvg;

/// Parses and inspects the normalized SVG tree produced by `usvg`.
mod usvg;

/// Runs the two SVG examples.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    resvg::run()?;
    usvg::run()?;
    Ok(())
}
