//! Runs the headless 2D raster graphics example.
//!
//! TODO Reviewed ??-??-2026.

mod tiny_skia;

/// Runs the 2D raster graphics example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tiny_skia::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
