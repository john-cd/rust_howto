//! Runs the artificial intelligence examples.
//!
//! TODO Reviewed ??-??-2026.

/// Robot path planning on an occupancy grid using the A* algorithm.
mod astar;
/// Controls a counter with a behavior tree built using `bonsai_bt`.
mod bonsai_bt;

/// Runs the artificial intelligence examples.
fn main() -> anyhow::Result<()> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("astar"));
    command = command.subcommand(clap::Command::new("bonsai-bt"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("astar", _)) => {
            astar::run()?;
            Ok(())
        }
        Some(("bonsai-bt", _)) => {
            bonsai_bt::run()?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
