//! Runs one of the `fuzzing` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

#[cfg(not(windows))]
mod afl;
mod bolero_example;

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
    /// Runs the `afl` example.
    #[cfg(not(windows))]
    #[command(name = "afl")]
    Afl,
    /// Runs the `bolero_example` example.
    #[command(name = "bolero_example")]
    BoleroExample,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(not(windows))]
            Commands::Afl => {
                afl::run();
            }
            Commands::BoleroExample => {
                bolero_example::run();
            }
        }
    }
}
