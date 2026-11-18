//! Runs the Polars DataFrame example.
//!
//! TODO Reviewed ??-??-2026.

/// Demonstrates lazy sales aggregation and a join against regional targets.
mod dataframes;

/// Runs the example from the `dataframes` module.
fn main() -> anyhow::Result<()> {
    dataframes::run()
}
