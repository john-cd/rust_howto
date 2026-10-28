// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to load an image, convert it to grayscale, and
//! save the result using the `opencv` crate.

#![cfg(all(target_os = "linux", feature = "opencv"))]

use std::fs;
use std::path::Path;

use image::Rgb;
use image::RgbImage;
use opencv::core::Mat;
use opencv::core::Scalar;
use opencv::core::Size;
use opencv::core::Vector;
use opencv::core::count_non_zero;
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

/// Creates a deterministic image, detects its edges, and saves the result.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = Path::new("temp/opencv");
    fs::create_dir_all(output_dir)?;
    let input_path = output_dir.join("input.png");
    let output_path = output_dir.join("edges.png");
    write_sample_image(&input_path)?;
    let edges = process_image(&input_path, &output_path)?;
    assert!(count_non_zero(&edges)? > 0);
    println!("Saved edge image to {}", output_path.display());
    Ok(())
}

/// Writes a black image with a bright rectangle for repeatable edge detection.
fn write_sample_image(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut image = RgbImage::from_pixel(100, 100, Rgb([0, 0, 0]));
    for y in 20..80 {
        for x in 20..80 {
            image.put_pixel(x, y, Rgb([255, 255, 255]));
        }
    }
    image.save(path)?;
    Ok(())
}

/// Loads an image, converts it to grayscale, blurs it, and writes Canny edges.
///
/// # Errors
///
/// Returns an error if the image cannot be read, processed, or written.
fn process_image(
    input_path: &Path,
    output_path: &Path,
) -> Result<Mat, Box<dyn std::error::Error>> {
    let input_path = input_path.to_string_lossy();
    let input = imgcodecs::imread(&input_path, imgcodecs::IMREAD_COLOR)?;
    if input.empty() {
        return Err(std::io::Error::other(
            "OpenCV could not load the input image",
        )
        .into());
    }

    let mut grayscale = Mat::default();
    imgproc::cvt_color(&input, &mut grayscale, imgproc::COLOR_BGR2GRAY, 0)?;
    let mut blurred = Mat::default();
    imgproc::gaussian_blur(
        &grayscale,
        &mut blurred,
        Size::new(5, 5),
        0.0,
        0.0,
        opencv::core::BORDER_DEFAULT,
    )?;
    let mut edges = Mat::default();
    imgproc::canny(&blurred, &mut edges, 50.0, 150.0, 3, false)?;

    let output_path = output_path.to_string_lossy();
    if !imgcodecs::imwrite(&output_path, &edges, &Vector::new())? {
        return Err(std::io::Error::other(
            "OpenCV could not write the edge image",
        )
        .into());
    }
    Ok(edges)
}
// ANCHOR_END: example

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that Canny processing writes a nonempty edge image.
    #[test]
    fn canny_processing_writes_detected_edges()
    -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let input_path = temp_dir.path().join("input.png");
        let output_path = temp_dir.path().join("edges.png");
        write_sample_image(&input_path)?;

        let edges = process_image(&input_path, &output_path)?;

        assert!(output_path.is_file());
        assert_eq!(edges.rows(), 100);
        assert_eq!(edges.cols(), 100);
        assert!(count_non_zero(&edges)? > 0);
        Ok(())
    }
}
