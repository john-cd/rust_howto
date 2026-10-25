//! Runs the images examples.
//!
//! TODO Reviewed ??-??-2026.

mod images;
mod resize;

/// Runs the images examples.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    resize::run();
    images::run()
}
