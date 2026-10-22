//! Runs one of the `json` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod json5;
mod serde_json;
mod simd_json;

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
    /// Runs the `json5` example.
    #[command(name = "json5")]
    Json5,
    /// Runs the `serde_json` example.
    #[command(name = "serde_json")]
    SerdeJson,
    /// Runs the `simd_json` example.
    #[command(name = "simd_json")]
    SimdJson,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Json5 => {
                json5::run();
            }
            Commands::SerdeJson => {
                serde_json::run().map_err(|e| anyhow::anyhow!("{e}"))?;
            }
            Commands::SimdJson => {
                simd_json::run();
            }
        }
    }
    Ok(())
}
