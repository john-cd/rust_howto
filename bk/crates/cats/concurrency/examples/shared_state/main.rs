//! Runs one of the `shared_state` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod arc_make_mut;
mod global_mut_state;
mod send_sync;
mod shared_state_mutex;
mod shared_state_parking_lot;
mod shared_state_parking_lot2;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[derive(Subcommand)]
enum Commands {
    /// Runs the `arc_make_mut` example.
    #[command(name = "arc_make_mut")]
    ArcMakeMut,
    /// Runs the `global_mut_state` example.
    #[command(name = "global_mut_state")]
    GlobalMutState,
    /// Runs the `send_sync` example.
    #[command(name = "send_sync")]
    SendSync,
    /// Runs the `shared_state_mutex` example.
    #[command(name = "shared_state_mutex")]
    SharedStateMutex,
    /// Runs the `shared_state_parking_lot` example.
    #[command(name = "shared_state_parking_lot")]
    SharedStateParkingLot,
    /// Runs the `shared_state_parking_lot2` example.
    #[command(name = "shared_state_parking_lot2")]
    SharedStateParkingLot2,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::ArcMakeMut => {
                arc_make_mut::run();
            }
            Commands::GlobalMutState => {
                global_mut_state::run()?;
            }
            Commands::SendSync => {
                send_sync::run()?;
            }
            Commands::SharedStateMutex => {
                shared_state_mutex::run()?;
            }
            Commands::SharedStateParkingLot => {
                shared_state_parking_lot::run();
            }
            Commands::SharedStateParkingLot2 => {
                shared_state_parking_lot2::run();
            }
        }
    }
    Ok(())
}

/// Parses the command line and runs the selected example.
fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &["arc_make_mut", "global_mut_state", "send_sync", "shared_state_mutex", "shared_state_parking_lot", "shared_state_parking_lot2"];

    /// The command-line definition is internally consistent.
    #[test]
    fn test_cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Each example is reachable through its own subcommand.
    #[test]
    fn test_every_subcommand_parses() {
        for name in SUBCOMMANDS {
            let cli = Cli::try_parse_from(["example", name])
                .unwrap_or_else(|e| panic!("`{name}` should parse: {e}"));
            assert!(cli.command.is_some(), "`{name}` selected no command");
        }
    }

    /// Subcommand names are unique.
    #[test]
    fn test_subcommand_names_are_unique() {
        let mut sorted = SUBCOMMANDS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), SUBCOMMANDS.len());
    }

    /// Unknown subcommands are rejected.
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }

    /// Running without a subcommand selects nothing and succeeds.
    #[test]
    fn test_no_subcommand_runs_nothing() {
        let cli = Cli::try_parse_from(["example"]).expect("no subcommand is accepted");
        assert!(cli.command.is_none());
        dispatch(cli).expect("dispatching should succeed");
    }
}
