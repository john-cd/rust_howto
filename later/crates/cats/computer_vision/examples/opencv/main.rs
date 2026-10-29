//! Runs the OpenCV example on Linux when the `opencv` feature is enabled.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(all(target_os = "linux", feature = "opencv"))]
mod faces;
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    #[cfg_attr(
        all(target_os = "linux", feature = "opencv"),
        ignore = "needs OpenCV and its sample data"
    )]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
