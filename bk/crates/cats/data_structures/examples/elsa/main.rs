//! Runs the append-only collection examples.
//!
//! TODO Reviewed ??-??-2026.

/// Demonstrates stable references in an `elsa::FrozenMap`.
mod frozen_map;
/// Demonstrates stable references in an `elsa::FrozenVec`.
mod frozen_vec;

/// Runs both `elsa` collection examples.
fn main() {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("frozen-map"));
    command = command.subcommand(clap::Command::new("frozen-vec"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("frozen-map", _)) => {
            frozen_map::run();
        }
        Some(("frozen-vec", _)) => {
            frozen_vec::run();
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
