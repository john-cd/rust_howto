// ANCHOR: example
#![cfg(all(target_os = "linux", feature = "opencv"))]
use std::fs;

use image::Rgb;
use image::RgbImage;
use opencv::core::Mat;
use opencv::core::Vector;
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;
use rand::Rng;

/// This example demonstrates how to load an image, convert it to grayscale, and
/// save the result using the `opencv` crate.
///
/// **Note:** `OpenCV` requires `clang`, which is not installed by default on
/// Windows.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prep: create a temporary directory
    if !fs::exists("temp")? {
        fs::create_dir("temp")?;
    }
    // Prep: create a random image
    let width = 100;
    let height = 100;
    let mut rng = rand::rng();
    let mut rgb_image = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let (r, g, b): (u8, u8, u8) = rng.random();
            rgb_image.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    rgb_image.save("temp/input.jpg")?;

    // Load an image from file
    let image = imgcodecs::imread("temp/input.jpg", imgcodecs::IMREAD_COLOR)?;
    if image.empty() {
        return Err("Failed to load image".into());
    }

    // Convert the image to grayscale
    let mut gray_image = Mat::default();
    imgproc::cvt_color(&image, &mut gray_image, imgproc::COLOR_BGR2GRAY, 0)?;

    // Save the grayscale image to file
    imgcodecs::imwrite("temp/output.jpg", &gray_image, &Vector::new())?;

    println!("Image processing completed successfully.");
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() {
    main().unwrap();
}
// [review; expand example; review https://blog.devgenius.io/rust-and-opencv-bb0467bf35ff](https://github.com/john-cd/rust_howto/issues/1079)
