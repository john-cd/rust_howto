//! Runs the `datafusion` query example.
//!
//! TODO Reviewed ??-??-2026.

/// This example registers an in-memory Arrow table and groups its rows with both SQL and
/// DataFusion's `DataFrame` API.
mod datafusion;

/// Runs the grouped queries and checks the results.
fn main() -> anyhow::Result<()> {
    datafusion::run()
}
