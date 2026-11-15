//! Runs the headless 2D raster graphics example.
//!
//! TODO Reviewed ??-??-2026.

/// Draw shapes into a pixel buffer with `tiny-skia`.
mod tiny_skia;

/// Runs the 2D raster graphics example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tiny_skia::run()
}

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
