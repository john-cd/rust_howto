#![allow(dead_code)]
// ANCHOR: example
//! Generate and save a simple RGB image using the `image` crate.
//!
//! This example builds a gradient image in memory and writes it to a PNG file.

use std::error::Error;
use std::path::PathBuf;

use image::ImageBuffer;
use image::Rgb;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let output_path = image_output_path_from_args().unwrap_or_else(|| {
        let default = PathBuf::from("generated_image.png");
        eprintln!("No output path provided, writing to {}", default.display());
        default
    });

    let width = 512;
    let height = 256;
    let img = build_gradient_image(width, height);

    img.save(&output_path)?;
    println!("Saved generated image to {}", output_path.display());
    println!("Dimensions: {}x{}", width, height);

    Ok(())
}

fn build_gradient_image(
    width: u32,
    height: u32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (x * 255 / (width - 1)) as u8;
        let g = (y * 255 / (height - 1)) as u8;
        let b = ((x + y) * 255 / (width + height - 2)) as u8;
        *pixel = Rgb([r, g, b]);
    }

    img
}

fn image_output_path_from_args() -> Option<PathBuf> {
    std::env::args_os().nth(1).map(PathBuf::from)
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_gradient_image_has_expected_pixel_values() {
        let img = build_gradient_image(3, 3);
        assert_eq!(img[(0, 0)], Rgb([0, 0, 0]));
        assert_eq!(img[(2, 0)], Rgb([255, 0, 90]));
        assert_eq!(img[(0, 2)], Rgb([0, 255, 90]));
        assert_eq!(img[(2, 2)], Rgb([255, 255, 255]));
    }
}
