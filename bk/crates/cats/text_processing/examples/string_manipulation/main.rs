//! Runs one of the `string_manipulation` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod heck;
mod indoc;
mod textwrap;

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
    /// Runs the `heck` example.
    #[command(name = "heck")]
    Heck,
    /// Runs the `indoc` example.
    #[command(name = "indoc")]
    Indoc,
    /// Runs the `textwrap` example.
    #[command(name = "textwrap")]
    Textwrap,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Heck => {
                heck::run();
            }
            Commands::Indoc => {
                indoc::run();
            }
            Commands::Textwrap => {
                textwrap::run();
            }
        }
    }
}
