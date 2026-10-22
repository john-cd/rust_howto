//! Runs one of the `web` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "dioxus")]
mod dioxus;
#[cfg(all(not(windows), feature = "tauri"))] // TODO review Windows support
mod tauri;

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
    /// Runs the `dioxus` example.
    #[command(name = "dioxus")]
    #[cfg(feature = "dioxus")]
    Dioxus,
    /// Runs the `tauri` example.
    #[cfg(all(not(windows), feature = "tauri"))]
    #[command(name = "tauri")]
    Tauri,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(feature = "dioxus")]
            Commands::Dioxus => {
                dioxus::run();
            }
            #[cfg(all(not(windows), feature = "tauri"))]
            Commands::Tauri => {
                tauri::run();
            }
        }
    }
}
