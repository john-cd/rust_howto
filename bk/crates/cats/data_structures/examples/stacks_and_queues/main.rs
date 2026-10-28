//! Runs one of the `stacks_and_queues` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod queue;
mod stack;

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
    /// Runs the `queue` example.
    #[command(name = "queue")]
    Queue,
    /// Runs the `stack` example.
    #[command(name = "stack")]
    Stack,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Queue => {
                queue::run();
            }
            Commands::Stack => {
                stack::run();
            }
        }
    }
}
