//! Runs one of the `functional_programming` examples, chosen on the command
//! line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod either;
mod frunk;
mod im;
mod itertools;
mod rpds;

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
    /// Runs the `either` example.
    #[command(name = "either")]
    Either,
    /// Runs the `frunk` example.
    #[command(name = "frunk")]
    Frunk,
    /// Runs the `im` example.
    #[command(name = "im")]
    Im,
    /// Runs the `itertools` example.
    #[command(name = "itertools")]
    Itertools,
    /// Runs the `rpds` example.
    #[command(name = "rpds")]
    Rpds,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Either => {
                either::run();
            }
            Commands::Frunk => {
                frunk::run();
            }
            Commands::Im => {
                im::run();
            }
            Commands::Itertools => {
                itertools::run();
            }
            Commands::Rpds => {
                rpds::run();
            }
        }
    }
}
