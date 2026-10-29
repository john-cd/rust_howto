//! Runs the Arrow data processing example when the `arrow` feature is enabled.
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;

#[cfg(feature = "arrow")]
mod arrow;

/// Runs the example if the `arrow` feature is enabled.
fn main() -> Result<()> {
    #[cfg(feature = "arrow")]
    arrow::run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
