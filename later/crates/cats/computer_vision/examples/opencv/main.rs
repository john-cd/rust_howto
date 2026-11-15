//! Runs the OpenCV example on Linux when the `opencv` feature is enabled.
//!
//! TODO Reviewed ??-??-2026.

/// Loads a Haar cascade and applies it to a generated grayscale frame.
#[cfg(all(target_os = "linux", feature = "opencv"))]
mod faces;

/// This example demonstrates how to load an image, convert it to grayscale, and
#[cfg(all(target_os = "linux", feature = "opencv"))]
mod opencv;

/// Runs the example if it is supported.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(all(target_os = "linux", feature = "opencv"))]
    {
        opencv::run()?;
        if std::env::var_os("OPENCV_HAAR_CASCADE").is_some() {
            faces::run()?;
        }
    }
    Ok(())
}
