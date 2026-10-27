//! Runs one of the `window_creation` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod baseview;
mod tao;
mod winit;

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
    /// Runs the `baseview` example.
    #[command(name = "baseview")]
    Baseview,
    /// Runs the `tao` example.
    #[command(name = "tao")]
    Tao,
    /// Runs the `winit` example.
    #[command(name = "winit")]
    Winit,
}

/// Runs the example selected on the command line.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Baseview => {
                baseview::run()?;
            }
            Commands::Tao => {
                tao::run()?;
            }
            Commands::Winit => {
                winit::run()?;
            }
        }
    }
    Ok(())
}
