//! Runs the OpenCV example on Linux when the `opencv` feature is enabled.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(all(target_os = "linux", feature = "opencv"))]
mod opencv;

/// Runs the example if it is supported.
fn main() {
    #[cfg(all(target_os = "linux", feature = "opencv"))]
    {
        opencv::run();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
