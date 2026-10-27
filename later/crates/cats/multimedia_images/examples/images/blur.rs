// ANCHOR: example
//! Blur an image in memory using a Gaussian filter.

use image::Rgb;
use image::RgbImage;
use image::imageops;

/// Applies a Gaussian blur to an image containing a bright pixel.
fn main() {
    let mut image = RgbImage::new(9, 9);
    image.put_pixel(4, 4, Rgb([255, 255, 255]));
    let blurred = imageops::blur(&image, 1.0);
    println!("Center after blur: {:?}", blurred.get_pixel(4, 4));
}
// ANCHOR_END: example

/// Runs the Gaussian blur example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that blur spreads light into neighboring pixels.
    #[test]
    fn blur_spreads_a_bright_pixel() {
        let mut image = RgbImage::new(9, 9);
        image.put_pixel(4, 4, Rgb([255, 255, 255]));
        let blurred = imageops::blur(&image, 1.0);
        assert!(blurred.get_pixel(4, 4)[0] < 255);
        assert!(blurred.get_pixel(4, 3)[0] > 0);
    }
}
