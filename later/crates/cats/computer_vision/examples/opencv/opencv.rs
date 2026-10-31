// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to load an image, convert it to grayscale, and
//! save the result using the `opencv` crate.
//!
//! OpenCV is the standard computer-vision library. Many algorithms work on
//! grayscale images, which keep one brightness value per pixel and are cheaper
//! to process than three colour channels. It must be installed on the system.

#![cfg(all(target_os = "linux", feature = "opencv"))]

use std::fs;
use std::path::Path;

use image::Rgb;
use image::RgbImage;
use opencv::core::Mat;
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
    let input = imgcodecs::imread(input_path, imgcodecs::IMREAD_COLOR)?;
    if input.empty() {
        return Err(std::io::Error::other(
            "OpenCV could not load the input image",
        )
        .into());
    }

    let grayscale = to_grayscale(&input)?;
    let edges = detect_edges(&grayscale)?;

    if !imgcodecs::imwrite(output_path, &edges, &Vector::new())? {
        return Err(std::io::Error::other(
            "OpenCV could not write the edge image",
        )
        .into());
    }
    Ok(edges)
}

// ANCHOR: grayscale
/// Converts a BGR image to one channel while retaining its dimensions.
fn to_grayscale(input: &Mat) -> opencv::Result<Mat> {
    let mut grayscale = Mat::default();
    imgproc::cvt_color(input, &mut grayscale, imgproc::COLOR_BGR2GRAY, 0)?;
    Ok(grayscale)
}
// ANCHOR_END: grayscale

// ANCHOR: canny
/// Smooths a grayscale frame and computes its Canny edge mask.
fn detect_edges(grayscale: &Mat) -> opencv::Result<Mat> {
    let mut blurred = Mat::default();
    imgproc::gaussian_blur(
        grayscale,
        &mut blurred,
        Size::new(5, 5),
        0.0,
        0.0,
        opencv::core::BORDER_DEFAULT,
    )?;
    let mut edges = Mat::default();
    imgproc::canny(&blurred, &mut edges, 50.0, 150.0, 3, false)?;

    Ok(edges)
}
// ANCHOR_END: canny
// ANCHOR_END: example

/// Runs the generated-image grayscale and edge-detection workflow.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use opencv::core::CV_8UC1;
    use opencv::core::CV_8UC3;
    use opencv::core::Scalar;

    use super::*;

    /// Converts a generated BGR frame without changing its dimensions.
    #[test]
    fn grayscale_preserves_generated_frame_dimensions() -> opencv::Result<()> {
        let input = Mat::new_rows_cols_with_default(
            24,
            32,
            CV_8UC3,
            Scalar::all(80.0),
        )?;
        let grayscale = to_grayscale(&input)?;
        assert_eq!((grayscale.rows(), grayscale.cols()), (24, 32));
        assert_eq!(grayscale.channels(), 1);
        Ok(())
    }

    /// Computes an empty edge mask for a generated constant grayscale frame.
    #[test]
    fn canny_preserves_dimensions_of_a_constant_frame() -> opencv::Result<()> {
        let input =
            Mat::new_rows_cols_with_default(24, 32, CV_8UC1, Scalar::all(0.0))?;
        let edges = detect_edges(&input)?;
        assert_eq!((edges.rows(), edges.cols()), (24, 32));
        assert_eq!(count_non_zero(&edges)?, 0);
        Ok(())
    }

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

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
