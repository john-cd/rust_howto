//! Runs one of the `user_directories` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod directories;
mod dirs;

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
    /// Runs the `directories` example.
    #[command(name = "directories")]
    Directories,
    /// Runs the `dirs` example.
    #[command(name = "dirs")]
    Dirs,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Directories => {
                directories::run();
            }
            Commands::Dirs => {
                dirs::run();
            }
        }
    }
}
