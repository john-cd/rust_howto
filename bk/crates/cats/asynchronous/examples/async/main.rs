//! Runs one of the `async` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod async1;
mod async2;

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
    /// Runs the `async1` example.
    #[command(name = "async1")]
    Async1,
    /// Runs the `async2` example.
    #[command(name = "async2")]
    Async2,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Async1 => {
                async1::run();
            }
            Commands::Async2 => {
                async2::run();
            }
        }
    }
}
