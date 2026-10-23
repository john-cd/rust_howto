//! Runs one of the `windows` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod winapi;
mod windows;

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
    /// Runs the `winapi` example.
    #[command(name = "winapi")]
    Winapi,
    /// Runs the `windows` example.
    #[command(name = "windows")]
    Windows,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Winapi => {
                winapi::run();
            }
            Commands::Windows => {
                windows::run();
            }
        }
    }
}
