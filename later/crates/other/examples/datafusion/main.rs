//! Runs the `datafusion` query example.
//!
//! TODO Reviewed ??-??-2026.

/// This example registers an in-memory Arrow table and groups its rows with both SQL and
/// DataFusion's `DataFrame` API.
mod datafusion;

/// Runs the DataFusion example when the feature is enabled.
fn main() -> anyhow::Result<()> {
    #[cfg(feature = "datafusion")]
    datafusion::run()?;
    Ok(())
}
