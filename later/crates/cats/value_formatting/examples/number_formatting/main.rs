//! Runs the number formatting example.
//!
//! TODO Reviewed ??-??-2026.

/// Format a byte count as a human-readable size.
mod humansize_example;
/// Formats an integer with the `itoa` crate.
mod itoa;
/// Converts floating-point values to decimal strings with the `ryu` crate.
mod ryu;
/// Demonstrates standard Rust format syntax and locale-aware integer grouping.
mod standard_and_localized;

/// Runs the number formatting example.
fn main() {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("humansize"));
    command = command.subcommand(clap::Command::new("itoa"));
    command = command.subcommand(clap::Command::new("ryu"));
    command = command.subcommand(clap::Command::new("standard-and-localized"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("humansize", _)) => {
            humansize_example::run();
        }
        Some(("itoa", _)) => {
            itoa::run();
        }
        Some(("ryu", _)) => {
            ryu::run();
        }
        Some(("standard-and-localized", _)) => {
            standard_and_localized::run();
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
