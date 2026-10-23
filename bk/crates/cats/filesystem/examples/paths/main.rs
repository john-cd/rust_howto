//! Runs one of the `paths` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod camino;
mod canonicalize;
mod manipulate_paths;

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
    /// Runs the `camino` example.
    #[command(name = "camino")]
    Camino,
    /// Runs the `canonicalize` example.
    #[command(name = "canonicalize")]
    Canonicalize,
    /// Runs the `manipulate_paths` example.
    #[command(name = "manipulate_paths")]
    ManipulatePaths,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Camino => {
                camino::run();
            }
            Commands::Canonicalize => {
                canonicalize::run()?;
            }
            Commands::ManipulatePaths => {
                manipulate_paths::run();
            }
        }
    }

    Ok(())
}
