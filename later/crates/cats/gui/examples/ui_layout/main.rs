//! Runs one of the `ui_layout` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod morphorm;
mod taffy;

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
    /// Runs the `morphorm` example.
    #[command(name = "morphorm")]
    Morphorm,
    /// Runs the `taffy` example.
    #[command(name = "taffy")]
    Taffy,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Morphorm => {
                morphorm::run();
            }
            Commands::Taffy => {
                taffy::run();
            }
        }
    }
}
