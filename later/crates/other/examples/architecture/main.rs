//! Runs one of the `architecture` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod cqrs;
mod di;
mod layered_architecture;
mod repository;
mod state_machine;

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
    /// Runs the `cqrs` example.
    #[command(name = "cqrs")]
    Cqrs,
    /// Runs the `di` example.
    #[command(name = "di")]
    Di,
    /// Runs the `layered_architecture` example.
    #[command(name = "layered_architecture")]
    LayeredArchitecture,
    /// Runs the `repository` example.
    #[command(name = "repository")]
    Repository,
    /// Runs the `state_machine` example.
    #[command(name = "state_machine")]
    StateMachine,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Cqrs => {
                cqrs::run()?;
            }
            Commands::Di => {
                di::run();
            }
            Commands::LayeredArchitecture => {
                layered_architecture::run()?;
            }
            Commands::Repository => {
                repository::run();
            }
            Commands::StateMachine => {
                state_machine::run();
            }
        }
    }
    Ok(())
}
