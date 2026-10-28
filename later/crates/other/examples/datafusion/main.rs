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
