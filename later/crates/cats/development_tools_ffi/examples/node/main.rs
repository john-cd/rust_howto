//! Runs one of the `node` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod napi;
mod neon;

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
    /// Runs the `napi` example.
    #[command(name = "napi")]
    Napi,
    /// Runs the `neon` example.
    #[command(name = "neon")]
    Neon,
}

/// Runs the example selected on the command line.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Napi => {
                napi::run();
            }
            Commands::Neon => {
                neon::run()?;
            }
        }
    }
    Ok(())
}
