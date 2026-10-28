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

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
