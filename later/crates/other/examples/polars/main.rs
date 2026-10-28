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
