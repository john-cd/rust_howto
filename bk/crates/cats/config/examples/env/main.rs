//! Configuration environment examples.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod dotenvy;
mod env;
mod envy;

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
    /// Runs the `dotenvy` example.
    #[command(name = "dotenvy")]
    Dotenvy,
    /// Runs the `env` example.
    #[command(name = "env")]
    Env,
    /// Runs the `envy` example.
    #[command(name = "envy")]
    Envy,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Dotenvy => {
                dotenvy::run()?;
            }
            Commands::Env => {
                env::run();
            }
            Commands::Envy => {
                envy::run();
            }
        }
    }

    Ok(())
}
