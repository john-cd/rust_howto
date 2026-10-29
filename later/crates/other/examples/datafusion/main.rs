//! Runs the `datafusion` query example.
//!
//! TODO Reviewed ??-??-2026.

mod datafusion;

/// Runs the DataFusion example when the feature is enabled.
fn main() -> anyhow::Result<()> {
    #[cfg(feature = "datafusion")]
    datafusion::run()?;
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
