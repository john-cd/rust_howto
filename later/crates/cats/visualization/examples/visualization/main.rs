//! Runs the visualization example.
//!
//! TODO Reviewed ??-??-2026.

mod plotly;

/// Runs the visualization example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    plotly::run().map_err(Into::into)
}
