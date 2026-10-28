// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Generate and save a simple RGB image using the `image` crate.
//!
//! This example builds a gradient image in memory and writes it to a PNG file.

use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

use image::ImageBuffer;
use image::Rgb;

/// Generates a gradient image and saves it to the path given on the command
/// line.
fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let output_path = image_output_path_from_args().unwrap_or_else(|| {
        let default = PathBuf::from("generated_image.png");
        eprintln!("No output path provided, writing to {}", default.display());
        default
    });

    save_gradient_image(&output_path, 512, 256)
}

/// Builds a gradient image of the given size and saves it to `output_path`.
fn save_gradient_image(
    output_path: &Path,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let img = build_gradient_image(width, height);

    img.save(output_path)?;
    println!("Saved generated image to {}", output_path.display());
    println!("Dimensions: {width}x{height}");

    Ok(())
}

/// Returns a gradient image of the given `width` and `height`.
fn build_gradient_image(
    width: u32,
    height: u32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(width, height);
    let max_x = u64::from(width.saturating_sub(1));
    let max_y = u64::from(height.saturating_sub(1));

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (u64::from(x) * 255 / max_x.max(1)) as u8;
        let g = (u64::from(y) * 255 / max_y.max(1)) as u8;
        let b = ((u64::from(x) + u64::from(y)) * 255 / (max_x + max_y).max(1))
            as u8;
        *pixel = Rgb([r, g, b]);
    }

    img
}

/// Returns the output path from the first command-line argument.
fn image_output_path_from_args() -> Option<PathBuf> {
    std::env::args_os().nth(1).map(PathBuf::from)
}
// ANCHOR_END: example

pub fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that gradient image round trips through png.
    #[test]
    fn gradient_image_round_trips_through_png()
    -> Result<(), Box<dyn Error + Send + Sync>> {
        let temp_dir = tempfile::tempdir()?;
        let output_path = temp_dir.path().join("gradient.png");
        save_gradient_image(&output_path, 3, 3)?;
        let decoded = image::open(output_path)?.to_rgb8();
        assert_eq!(decoded.dimensions(), (3, 3));
        assert_eq!(decoded[(0, 0)], Rgb([0, 0, 0]));
        assert_eq!(decoded[(2, 2)], Rgb([255, 255, 255]));
        Ok(())
    }

    /// Verifies that build gradient image has expected pixel values.
    #[test]
    fn build_gradient_image_has_expected_pixel_values() {
        let img = build_gradient_image(3, 3);
        assert_eq!(img[(0, 0)], Rgb([0, 0, 0]));
        assert_eq!(img[(2, 0)], Rgb([255, 0, 127]));
        assert_eq!(img[(0, 2)], Rgb([0, 255, 127]));
        assert_eq!(img[(2, 2)], Rgb([255, 255, 255]));
    }

    /// Verifies that single pixel gradient is black.
    #[test]
    fn single_pixel_gradient_is_black() {
        let img = build_gradient_image(1, 1);
        assert_eq!(img[(0, 0)], Rgb([0, 0, 0]));
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
