//! Runs one of the `rust_specific_patterns` examples, chosen on the command
//! line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod dyn_clone;
mod pin_project;

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
    /// Runs the `dyn_clone` example.
    #[command(name = "dyn_clone")]
    DynClone,
    /// Runs the `pin_project` example.
    #[command(name = "pin_project")]
    PinProject,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::DynClone => {
                dyn_clone::run();
            }
            Commands::PinProject => {
                pin_project::run();
            }
        }
    }
}
