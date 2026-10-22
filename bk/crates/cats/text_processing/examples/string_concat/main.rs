//! Runs one of the `string_concat` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod string_concat;
mod string_concat2;

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
    /// Runs the `string_concat` example.
    #[command(name = "string_concat")]
    StringConcat,
    /// Runs the `string_concat2` example.
    #[command(name = "string_concat2")]
    StringConcat2,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::StringConcat => {
                string_concat::run();
            }
            Commands::StringConcat2 => {
                string_concat2::run();
            }
        }
    }
}
