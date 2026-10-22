//! Runs one of the `shared_state` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

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
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::ArcMakeMut => {
                arc_make_mut::run();
            }
            Commands::GlobalMutState => {
                global_mut_state::run()?;
            }
            Commands::SendSync => {
                send_sync::run();
            }
            Commands::SharedStateMutex => {
                shared_state_mutex::run();
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
