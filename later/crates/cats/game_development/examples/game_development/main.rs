//! Runs one of the `game_development` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod game_development1;
mod glam;

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
    /// Runs the `game_development1` example.
    #[command(name = "game_development1")]
    GameDevelopment1,
    /// Runs the `glam` example.
    #[command(name = "glam")]
    Glam,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::GameDevelopment1 => {
                game_development1::run();
            }
            Commands::Glam => {
                glam::run();
            }
        }
    }
}
