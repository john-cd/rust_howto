//! Runs one of the `middleware` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod cors;
mod tower_http;

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
    /// Runs the `cors` example.
    #[command(name = "cors")]
    Cors,
    /// Runs the `tower_http` example.
    #[command(name = "tower_http")]
    TowerHttp,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Cors => {
                cors::run()?;
            }
            Commands::TowerHttp => {
                tower_http::run()?;
            }
        }
    }
    Ok(())
}
