//! Runs the SVG parsing and raster-rendering examples.
//!
//! TODO Reviewed ??-??-2026.

mod resvg;
mod usvg;

/// Runs the two SVG examples.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    resvg::run()?;
    usvg::run()?;
    Ok(())
}
