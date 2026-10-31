//! Example of writing a cargo subcommand.
//!
//! TODO Reviewed ??-??-2026.

mod cargo_subcommand;

/// Runs the `cargo-hello` subcommand with the arguments supplied by Cargo.
///
/// # Errors
///
/// Returns an error if Cargo cannot read metadata for the selected workspace.
fn main() -> anyhow::Result<()> {
    cargo_subcommand::run_from(std::env::args_os())
}

#[cfg(test)]
mod tests {
    /// Runs the plugin against a temporary Cargo package; the plugin reads
    /// the process arguments in `main`, which belong to the test harness.
    #[test]
    fn test_plugin_runs_against_a_temporary_package() {
        assert!(super::cargo_subcommand::run().is_ok());
    }
}
