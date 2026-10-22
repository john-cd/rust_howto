//! Runs one of the `futures` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod futures1;
mod futures2;
mod futures3;
mod futures_util;

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
    /// Runs the `futures1` example.
    #[command(name = "futures1")]
    Futures1,
    /// Runs the `futures2` example.
    #[command(name = "futures2")]
    Futures2,
    /// Runs the `futures3` example.
    #[command(name = "futures3")]
    Futures3,
    /// Runs the `futures_util` example.
    #[command(name = "futures_util")]
    FuturesUtil,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Futures1 => {
                futures1::run();
            }
            Commands::Futures2 => {
                futures2::run();
            }
            Commands::Futures3 => {
                futures3::run()?;
            }
            Commands::FuturesUtil => {
                futures_util::run();
            }
        }
    }
    Ok(())
}
