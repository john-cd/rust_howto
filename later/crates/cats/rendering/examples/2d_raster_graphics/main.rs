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

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
