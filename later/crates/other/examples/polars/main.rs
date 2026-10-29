//! Runs the `polars` data frame example.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(feature = "polars")]
mod polars;

/// Runs the example from the `polars` module.
fn main() -> anyhow::Result<()> {
    #[cfg(feature = "polars")]
    polars::run()?;
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
