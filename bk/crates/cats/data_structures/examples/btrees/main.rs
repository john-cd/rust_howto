//! Runs one of the `btrees` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod btreemap;
mod btreeset;

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
    /// Runs the `btreemap` example.
    #[command(name = "btreemap")]
    Btreemap,
    /// Runs the `btreeset` example.
    #[command(name = "btreeset")]
    Btreeset,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Btreemap => {
                btreemap::run();
            }
            Commands::Btreeset => {
                btreeset::run();
            }
        }
    }
}
