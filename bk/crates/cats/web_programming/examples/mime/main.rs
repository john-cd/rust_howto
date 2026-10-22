//! Runs one of the `mime` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

mod filename;
mod request;
mod string;

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
    /// Runs the `filename` example.
    #[command(name = "filename")]
    Filename,
    /// Runs the `request` example.
    #[command(name = "request")]
    Request,
    /// Runs the `string` example.
    #[command(name = "string")]
    String,
}

/// Runs the example selected on the command line.
fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Filename => {
                filename::run();
            }
            Commands::Request => {
                request::run()?;
            }
            Commands::String => {
                string::run();
            }
        }
    }

    Ok(())
}
