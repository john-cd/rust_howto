//! Runs the Arrow data processing example when the `arrow` feature is enabled.
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;

#[cfg(feature = "arrow")]
mod arrow;

/// Runs the example if the `arrow` feature is enabled.
fn main() -> Result<()> {
    #[cfg(feature = "arrow")]
    arrow::run()?;
    Ok(())
}
