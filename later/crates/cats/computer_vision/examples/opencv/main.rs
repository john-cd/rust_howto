//! Runs the OpenCV example on Linux when the `opencv` feature is enabled.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(all(target_os = "linux", feature = "opencv"))]
mod opencv;

/// Runs the example if it is supported.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(all(target_os = "linux", feature = "opencv"))]
    {
        opencv::run()?;
    }
    Ok(())
}
