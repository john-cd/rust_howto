//! Runs one of the `compression` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod async_compression;
mod async_zip;
mod flate2;
mod zip;

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
    /// Runs the `async_compression` example.
    #[command(name = "async_compression")]
    AsyncCompression,
    /// Runs the `async_zip` example.
    #[command(name = "async_zip")]
    AsyncZip,
    /// Runs the `flate2` example.
    #[command(name = "flate2")]
    Flate2,
    /// Runs the `zip` example.
    #[command(name = "zip")]
    Zip,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AsyncCompression => {
                async_compression::run()?;
            }
            Commands::AsyncZip => {
                async_zip::run()?;
            }
            Commands::Flate2 => {
                flate2::run()?;
            }
            Commands::Zip => {
                zip::run()?;
            }
        }
    }

    Ok(())
}
