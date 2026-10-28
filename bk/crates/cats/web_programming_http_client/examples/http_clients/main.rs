//! Runs one of the `http_clients` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod hyper;
mod ureq;

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
    /// Runs the `hyper` example.
    #[command(name = "hyper")]
    Hyper,
    /// Runs the `ureq` example.
    #[command(name = "ureq")]
    Ureq,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Hyper => {
                hyper::run()?;
            }
            Commands::Ureq => {
                ureq::run()?;
            }
        }
    }

    Ok(())
}
