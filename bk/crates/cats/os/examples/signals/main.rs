//! Runs the graceful shutdown examples.
//!
//! TODO Reviewed ??-??-2026.

mod ctrlc_example;
mod tokio_signal;

/// Runs both examples.
fn main() -> anyhow::Result<()> {
    ctrlc_example::run()?;
    tokio_signal::run()?;
    Ok(())
}
