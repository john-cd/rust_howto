//! Runs one of the `mocking` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod faux;
mod mockall;

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
    /// Runs the `faux` example.
    #[command(name = "faux")]
    Faux,
    /// Runs the `mockall` example.
    #[command(name = "mockall")]
    Mockall,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Faux => {
                faux::run();
            }
            Commands::Mockall => {
                mockall::run();
            }
        }
    }
}
