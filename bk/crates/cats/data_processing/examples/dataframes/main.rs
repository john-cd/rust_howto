//! Runs the Polars DataFrame example.
//!
//! TODO Reviewed ??-??-2026.

mod dataframes;

/// Runs the example from the `dataframes` module.
fn main() -> anyhow::Result<()> {
    dataframes::run()
}
