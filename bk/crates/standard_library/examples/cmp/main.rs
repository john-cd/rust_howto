//! Runs one of the `cmp` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod eq;
mod ord;

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
    /// Runs the `eq` example.
    #[command(name = "eq")]
    Eq,
    /// Runs the `ord` example.
    #[command(name = "ord")]
    Ord,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Eq => {
                eq::run();
            }
            Commands::Ord => {
                ord::run();
            }
        }
    }
}
