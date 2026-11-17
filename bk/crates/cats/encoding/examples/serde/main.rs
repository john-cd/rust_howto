//! Runs one of the `serde` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// This example demonstrates parsing a JSON string into a `serde_json::Value`
mod json;
/// This example demonstrates how to use the `monostate` crate with `serde` to
mod monostate;
/// Demonstrates basic serialization and deserialization using `serde`.
mod serde;
/// `serde_ignored` is a crate that helps handle unknown fields when deserializing with `serde`.
mod serde_ignored;
/// Serializing and deserializing a struct to and from JSON with `serde_json`.
mod serde_json;

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
    /// Runs the `json` example.
    #[command(name = "json")]
    Json,

    /// Runs the `monostate` example.
    #[command(name = "monostate")]
    Monostate,

    /// Runs the `serde` example.
    #[command(name = "serde")]
    Serde,

    /// Runs the `serde_ignored` example.
    #[command(name = "serde_ignored")]
    SerdeIgnored,

    /// Runs the `serde_json` example.
    #[command(name = "serde_json")]
    SerdeJson,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Json => {
                json::run()?;
            }
            Commands::Monostate => {
                monostate::run();
            }
            Commands::Serde => {
                serde::run();
            }
            Commands::SerdeIgnored => {
                serde_ignored::run();
            }
            Commands::SerdeJson => {
                serde_json::run();
            }
        }
    }
    Ok(())
}

/// Parses the command line and runs the selected example.
fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &["json", "monostate", "serde", "serde_ignored", "serde_json"];

    /// Confirms that the command and its arguments are valid.
    #[test]
    fn test_cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Each example is reachable through its own subcommand.
    #[test]
    fn test_every_subcommand_parses() {
        for name in SUBCOMMANDS {
            let cli = Cli::try_parse_from(["example", name])
                .unwrap_or_else(|e| panic!("`{name}` should parse: {e}"));
            assert!(cli.command.is_some(), "`{name}` selected no command");
        }
    }

    /// Subcommand names are unique.
    #[test]
    fn test_subcommand_names_are_unique() {
        let mut sorted = SUBCOMMANDS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), SUBCOMMANDS.len());
    }

    /// Unknown subcommands are rejected.
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }
}
