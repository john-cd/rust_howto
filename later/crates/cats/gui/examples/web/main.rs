//! Runs one of the `web` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "dioxus")]
mod dioxus;
#[cfg(feature = "tauri")]
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
    /// Runs the plain Rust command example used by a Tauri backend.
    #[cfg(feature = "tauri")]
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
            #[cfg(feature = "tauri")]
            Commands::Tauri => {
                tauri::run();
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
