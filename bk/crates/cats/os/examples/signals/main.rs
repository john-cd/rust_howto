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

/// Verifies that the graceful shutdown examples.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "the Ctrl-C handler can be registered only once per process, and `ctrlc_example::tests::run_completes` already does so"]
    fn main_runs() {
        main().expect("the example should run");
    }
}
