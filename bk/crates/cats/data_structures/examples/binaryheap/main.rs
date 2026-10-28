//! Runs one of the `binaryheap` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod binaryheap;
mod priority_queue;

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
    /// Runs the `binaryheap` example.
    #[command(name = "binaryheap")]
    Binaryheap,
    /// Runs the `priority_queue` example.
    #[command(name = "priority_queue")]
    PriorityQueue,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Binaryheap => {
                binaryheap::run();
            }
            Commands::PriorityQueue => {
                priority_queue::run();
            }
        }
    }
}
