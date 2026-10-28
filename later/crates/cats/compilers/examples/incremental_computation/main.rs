//! Runs one of the `incremental_computation` examples, chosen on the command
//! line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod comemo;
mod salsa;

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
    /// Runs the `comemo` example.
    #[command(name = "comemo")]
    Comemo,
    /// Runs the `salsa` example.
    #[command(name = "salsa")]
    Salsa,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Comemo => {
                comemo::run();
            }
            Commands::Salsa => {
                salsa::run();
            }
        }
    }
}
