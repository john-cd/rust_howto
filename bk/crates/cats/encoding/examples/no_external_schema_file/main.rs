//! Runs one of the `no_external_schema_file` examples, chosen on the command
//! line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod postcard;
mod rkyv;

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
    /// Runs the `postcard` example.
    #[command(name = "postcard")]
    Postcard,
    /// Runs the `rkyv` example.
    #[command(name = "rkyv")]
    Rkyv,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Postcard => {
                postcard::run();
            }
            Commands::Rkyv => {
                rkyv::run()?;
            }
        }
    }
    Ok(())
}
