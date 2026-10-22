//! Runs one of the `streams` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod streams;
mod streams2;
mod streams3;

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
    /// Runs the `streams` example.
    #[command(name = "streams")]
    Streams,
    /// Runs the `streams2` example.
    #[command(name = "streams2")]
    Streams2,
    /// Runs the `streams3` example.
    #[command(name = "streams3")]
    Streams3,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Streams => {
                streams::run();
            }
            Commands::Streams2 => {
                streams2::run()?;
            }
            Commands::Streams3 => {
                streams3::run();
            }
        }
    }
    Ok(())
}
