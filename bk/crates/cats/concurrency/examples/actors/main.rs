//! Runs one of the `actors` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod actix;
mod actors;
mod ractor;
#[cfg(feature = "riker")]
mod riker;
#[cfg(feature = "stakker")]
mod stakker;

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
    /// Runs the `actix` example.
    #[command(name = "actix")]
    Actix,
    /// Runs the `actors` example.
    #[command(name = "actors")]
    Actors,
    /// Runs the `ractor` example.
    #[command(name = "ractor")]
    Ractor,
    /// Runs the `riker` example.
    #[cfg(feature = "riker")]
    #[command(name = "riker")]
    Riker,
    /// Runs the `stakker` example.
    #[cfg(feature = "stakker")]
    #[command(name = "stakker")]
    Stakker,
}

/// Runs the example selected on the command line.
///
/// # Errors
/// Returns an error if the selected Riker example cannot start.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Actix => {
                actix::run();
            }
            Commands::Actors => {
                actors::run();
            }
            Commands::Ractor => {
                ractor::run();
            }
            #[cfg(feature = "riker")]
            Commands::Riker => {
                riker::run()?;
            }
            #[cfg(feature = "stakker")]
            Commands::Stakker => {
                stakker::run();
            }
        }
    }

    Ok(())
}
