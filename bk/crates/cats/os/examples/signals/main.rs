//! Runs the graceful shutdown examples.
//!
//! TODO Reviewed ??-??-2026.

/// Shut a worker thread down cleanly when the user presses Ctrl-C.
mod ctrlc_example;
/// Shut an async service down on Ctrl-C or SIGTERM with Tokio.
mod tokio_signal;

/// Runs both examples.
fn main() -> anyhow::Result<()> {
    ctrlc_example::run()?;
    tokio_signal::run()?;
    Ok(())
}
