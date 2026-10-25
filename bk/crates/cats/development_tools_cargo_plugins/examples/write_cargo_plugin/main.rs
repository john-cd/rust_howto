//! Example of writing a cargo subcommand.
//!
//! TODO Reviewed ??-??-2026.

mod cargo_subcommand;

/// Runs the example.
fn main() -> anyhow::Result<()> {
    cargo_subcommand::run()
}
