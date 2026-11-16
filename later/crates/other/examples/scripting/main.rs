//! Runs the scripting example.
//!
//! TODO Reviewed ??-??-2026.

/// Demonstrates embedding Rhai scripts in a Rust application.
mod rhai;
/// Expose a Rust type to Rhai, keep state in a `Scope` and sandbox scripts.
mod rhai_types_and_limits;

/// Runs the scripting example.
fn main() -> anyhow::Result<()> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("rhai"));
    command = command.subcommand(clap::Command::new("rhai-types-and-limits"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("rhai", _)) => {
            rhai::run().map_err(|error| anyhow::anyhow!(error.to_string()))?;
            Ok(())
        }
        Some(("rhai-types-and-limits", _)) => {
            rhai_types_and_limits::run().map_err(|error| anyhow::anyhow!(error.to_string()))?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
