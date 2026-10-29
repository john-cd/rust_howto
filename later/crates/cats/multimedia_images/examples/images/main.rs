//! Runs the images examples.
//!
//! TODO Reviewed ??-??-2026.

mod blur;
mod convert;
mod exif_metadata;
mod images;
mod optimize_png;
mod resize;

/// Runs the images examples.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    blur::run();
    convert::run()?;
    exif_metadata::run()?;
    optimize_png::run()?;
    resize::run();
    images::run()
}
