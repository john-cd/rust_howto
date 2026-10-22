//! Runs one of the `ml` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "candle")]
mod candle;
mod linfa;
mod smartcore;

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
    /// Runs the `candle` example.
    #[cfg(feature = "candle")]
    #[command(name = "candle")]
    Candle,
    /// Runs the `linfa` example.
    #[command(name = "linfa")]
    Linfa,
    /// Runs the `smartcore` example.
    #[command(name = "smartcore")]
    Smartcore,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(feature = "candle")]
            Commands::Candle => {
                candle::run();
            }
            Commands::Linfa => {
                linfa::run();
            }
            Commands::Smartcore => {
                smartcore::run();
            }
        }
    }
}
