//! Runs the INI parser example.
//!
//! TODO Reviewed ??-??-2026.

/// Turns ini input into structured syntax data while preserving source locations and reporting
/// malformed input. Analysis and validation code use these routines before applying domain-specific
/// rules.
mod configparser_ini;
/// This example demonstrates how to use `rust-ini` to parse and manipulate INI
mod rust_ini;

/// Runs the ini example.
fn main() -> anyhow::Result<()> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("configparser-ini"));
    command = command.subcommand(clap::Command::new("rust-ini"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("configparser-ini", _)) => {
            configparser_ini::run()?;
            Ok(())
        }
        Some(("rust-ini", _)) => {
            rust_ini::run()?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
