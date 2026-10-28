//! Runs one of the `connection_pool` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod deadpool;
#[cfg(feature = "postgres")]
mod deadpool2;

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
    /// Runs the `deadpool` example.
    #[command(name = "deadpool")]
    Deadpool,
    /// Runs the `deadpool2` example.
    #[cfg(feature = "postgres")]
    #[command(name = "deadpool2")]
    Deadpool2,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Deadpool => {
                deadpool::run()?;
            }
            #[cfg(feature = "postgres")]
            Commands::Deadpool2 => {
                deadpool2::run()?;
            }
        }
    }
    Ok(())
}
