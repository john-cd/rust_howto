//! Loads a Haar cascade and applies it to a generated grayscale frame.

// ANCHOR: example
use std::error::Error;

use opencv::core::CV_8UC1;
use opencv::core::Mat;
use opencv::core::Rect;
use opencv::core::Scalar;
use opencv::core::Size;
use opencv::core::Vector;
use opencv::objdetect::CascadeClassifier;
use opencv::prelude::*;

/// Loads a caller-selected model and detects faces in a blank test frame.
fn main() -> Result<(), Box<dyn Error>> {
    let model_path = std::env::var("OPENCV_HAAR_CASCADE")?;
    let mut classifier = CascadeClassifier::new(&model_path)?;
    let frame =
        Mat::new_rows_cols_with_default(64, 64, CV_8UC1, Scalar::all(0.0))?;
    let mut faces = Vector::<Rect>::new();
    classifier.detect_multi_scale(
        &frame,
        &mut faces,
        1.1,
        3,
        0,
        Size::new(16, 16),
        Size::default(),
    )?;
    assert_eq!((frame.rows(), frame.cols()), (64, 64));
    assert!(
        faces.is_empty(),
        "a blank frame should contain no detected faces"
    );
    println!("Detected {} faces", faces.len());
    Ok(())
}
// ANCHOR_END: example

/// Runs face detection using the model selected at the entry point.
pub fn run() -> Result<(), Box<dyn Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks a loaded classifier against a generated image with no faces.
    #[test]
    #[ignore = "needs OpenCV and a cascade XML path in OPENCV_HAAR_CASCADE"]
    fn generated_blank_frame_contains_no_faces() -> Result<(), Box<dyn Error>> {
        main()
    }
}
