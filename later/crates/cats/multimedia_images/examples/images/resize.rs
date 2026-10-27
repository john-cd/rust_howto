#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Create an image in memory, resize it and convert it to grayscale.

use image::DynamicImage;
use image::RgbImage;
use image::imageops;
use image::imageops::FilterType;

/// Builds a 64x32 gradient, shrinks it by half and makes it grayscale.
fn main() {
    // Every pixel gets a color computed from its position.
    let image = RgbImage::from_fn(64, 32, |x, y| {
        image::Rgb([(x * 4) as u8, (y * 8) as u8, 128])
    });

    // `Lanczos3` gives the best quality when downscaling; `Nearest` is fastest.
    let small = imageops::resize(&image, 32, 16, FilterType::Lanczos3);
    assert_eq!(small.dimensions(), (32, 16));

    let gray = DynamicImage::ImageRgb8(small).into_luma8();
    assert_eq!(gray.dimensions(), (32, 16));
    println!("{:?}", gray.dimensions());
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the resize example through its entry point.
    #[test]
    fn test_resize_example_entry_point() {
        main();
    }
}
