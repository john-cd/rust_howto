//! Runs one of the `concurrent_data_structures` examples, chosen on the command
//! line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod crossbeam_queue;
mod dashmap;
mod flurry;
mod papaya;

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
    /// Runs the `crossbeam_queue` example.
    #[command(name = "crossbeam_queue")]
    CrossbeamQueue,
    /// Runs the `dashmap` example.
    #[command(name = "dashmap")]
    Dashmap,
    /// Runs the `flurry` example.
    #[command(name = "flurry")]
    Flurry,
    /// Runs the `papaya` example.
    #[command(name = "papaya")]
    Papaya,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CrossbeamQueue => {
                crossbeam_queue::run();
            }
            Commands::Dashmap => {
                dashmap::run();
            }
            Commands::Flurry => {
                flurry::run();
            }
            Commands::Papaya => {
                papaya::run();
            }
        }
    }
}
