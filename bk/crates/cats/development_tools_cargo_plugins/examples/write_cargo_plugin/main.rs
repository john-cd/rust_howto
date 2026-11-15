//! Example of writing a cargo subcommand.
//!
//! TODO Reviewed ??-??-2026.

/// Parse the arguments of a `cargo hello` subcommand and select packages.
mod cargo_subcommand;

/// Runs the `cargo-hello` subcommand with the arguments supplied by Cargo.
///
/// # Errors
///
/// Returns an error if Cargo cannot read metadata for the selected workspace.
fn main() -> anyhow::Result<()> {
    cargo_subcommand::run_from(std::env::args_os())
}
