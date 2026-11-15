//! Runs the Elasticsearch example when the `elasticsearch` feature is enabled.
//!
//! TODO Reviewed ??-??-2026.

/// This module provides examples of interacting with Elasticsearch using the
#[cfg(feature = "elasticsearch")]
mod elasticsearch;

/// Runs the example if the `elasticsearch` feature is enabled.
fn main() -> anyhow::Result<()> {
    #[cfg(feature = "elasticsearch")]
    {
        elasticsearch::run()?;
    }
    Ok(())
}
