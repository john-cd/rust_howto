// TODO Reviewed ??-??-2026.

//! Runs examples of explicit integer overflow behavior.

/// Demonstrates explicit primitive integer overflow operations.
mod overflow_methods;
/// Demonstrates wrapping arithmetic with `std::num::Wrapping`.
mod wrapping_type;
/// Shows the default overflow behavior for the active Cargo profile.
mod overflow_profile;

/// Runs one explicit integer-overflow example.
fn main() {
    let mut command = clap::Command::new("overflow");
    command = command.subcommand(clap::Command::new("overflow-methods"));
    command = command.subcommand(clap::Command::new("wrapping-type"));
    command = command.subcommand(clap::Command::new("profile-behavior"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("overflow-methods", _)) => overflow_methods::run(),
        Some(("wrapping-type", _)) => wrapping_type::run(),
        Some(("profile-behavior", _)) => overflow_profile::run(),
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
