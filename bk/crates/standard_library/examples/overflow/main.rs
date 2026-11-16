//! Runs examples of explicit integer overflow behavior.
//!
//! TODO Reviewed ??-??-2026.

/// Demonstrates explicit primitive integer overflow operations.
mod overflow_methods;
/// Demonstrates wrapping arithmetic with `std::num::Wrapping`.
mod wrapping_type;

/// Runs the primitive methods and `Wrapping` examples.
fn main() {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("overflow-methods"));
    command = command.subcommand(clap::Command::new("wrapping-type"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("overflow-methods", _)) => {
            overflow_methods::run();
        }
        Some(("wrapping-type", _)) => {
            wrapping_type::run();
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
