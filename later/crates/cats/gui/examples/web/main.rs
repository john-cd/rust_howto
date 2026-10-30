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
    use clap::CommandFactory as _;

    use super::*;

    /// The command-line definition is internally consistent.
    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Running without a subcommand is accepted and runs nothing.
    #[test]
    fn missing_subcommand_is_accepted() {
        assert!(Cli::try_parse_from(["example"]).is_ok());
    }

    /// Every declared subcommand name is accepted.
    #[test]
    fn declared_subcommands_parse() {
        let command = Cli::command();
        let names: Vec<_> = command
            .get_subcommands()
            .map(clap::Command::get_name)
            .collect();
        for name in names {
            assert!(
                Cli::try_parse_from(["example", name]).is_ok(),
                "subcommand `{name}` should parse"
            );
        }
    }

    /// Unknown subcommands are rejected.
    #[test]
    fn unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }
}
