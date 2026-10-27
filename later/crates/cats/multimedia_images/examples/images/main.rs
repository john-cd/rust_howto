//! Runs the images examples.
//!
//! TODO Reviewed ??-??-2026.

mod blur;
mod convert;
mod images;
mod resize;

/// Runs the images examples.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    blur::run();
    convert::run()?;
    resize::run();
    images::run()
}
