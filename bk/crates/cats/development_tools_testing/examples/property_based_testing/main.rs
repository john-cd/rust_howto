//! Runs one of the `property_based_testing` examples, chosen on the command
//! line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod fake;
mod proptest;

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
    /// Runs the `fake` example.
    #[command(name = "fake")]
    Fake,
    /// Runs the `proptest` example.
    #[command(name = "proptest")]
    Proptest,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Fake => {
                fake::run();
            }
            Commands::Proptest => {
                proptest::run();
            }
        }
    }
}
