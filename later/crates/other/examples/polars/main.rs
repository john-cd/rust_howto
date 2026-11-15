//! Runs the `polars` data frame example.
//!
//! TODO Reviewed ??-??-2026.

/// Example: query inline sales data with Polars and join regional targets.
#[cfg(feature = "polars")]
mod polars;

/// Runs the example from the `polars` module.
fn main() -> anyhow::Result<()> {
    #[cfg(feature = "polars")]
    polars::run()?;
    Ok(())
}
