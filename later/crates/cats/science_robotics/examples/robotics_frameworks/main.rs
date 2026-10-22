//! Runs one of the `robotics_frameworks` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod openrr;
mod zenoh;

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
    /// Runs the `openrr` example.
    #[command(name = "openrr")]
    Openrr,
    /// Runs the `zenoh` example.
    #[command(name = "zenoh")]
    Zenoh,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Openrr => {
                openrr::run();
            }
            Commands::Zenoh => {
                zenoh::run()?;
            }
        }
    }
    Ok(())
}
