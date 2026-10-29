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

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    #[ignore = "the Ctrl-C handler can be registered only once per process, and `ctrlc_example::tests::run_completes` already does so"]
    fn main_runs() {
        main().expect("the example should run");
    }
}
