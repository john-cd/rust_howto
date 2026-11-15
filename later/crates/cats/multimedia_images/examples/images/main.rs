//! Runs the images examples.
//!
//! TODO Reviewed ??-??-2026.

/// Blur an image in memory using a Gaussian filter.
mod blur;
/// Encode an image to PNG and JPEG in memory.
mod convert;
/// Reads an orientation tag from a generated TIFF/EXIF record.
mod exif_metadata;
/// Generate and save a simple RGB image using the `image` crate.
mod images;
/// Optimizes a generated PNG and verifies its decoded pixels.
mod optimize_png;
/// Create an image in memory, resize it and convert it to grayscale.
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

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
