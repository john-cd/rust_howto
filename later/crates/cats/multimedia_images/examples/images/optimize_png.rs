//! Optimizes a generated PNG and verifies its decoded pixels.
//!
//! PNG optimizers try many compression settings and filters to produce a
//! smaller file with exactly the same pixels, which saves bandwidth on web
//! pages. The check at the end proves the result is lossless.

// ANCHOR: example
use std::io::Cursor;

use image::DynamicImage;
use image::ImageFormat;
use image::Rgb;
use image::RgbImage;

/// Recompresses a PNG without changing the decoded image.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pixels = RgbImage::from_pixel(32, 24, Rgb([12, 34, 56]));
    let original = DynamicImage::ImageRgb8(pixels.clone());
    let mut input = Cursor::new(Vec::new());
    original.write_to(&mut input, ImageFormat::Png)?;
    let optimized = oxipng::optimize_from_memory(
        input.get_ref(),
        &oxipng::Options::default(),
    )?;
    let decoded =
        image::load_from_memory_with_format(&optimized, ImageFormat::Png)?;
    assert_eq!(decoded.to_rgb8(), pixels);
    println!(
        "PNG bytes: {} -> {}",
        input.get_ref().len(),
        optimized.len()
    );
    Ok(())
}
// ANCHOR_END: example

/// Runs the PNG recompression recipe.
pub fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    main()
}

#[cfg(test)]
mod tests {
    /// Verifies lossless recompression by comparing every decoded pixel.
    #[test]
    fn optimization_preserves_generated_pixels()
    -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        super::main()
    }
}
