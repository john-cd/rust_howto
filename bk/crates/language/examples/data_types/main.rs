//! Runs one of the `data_types` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Demonstrates the use of tuples and arrays in Rust, including their declaration, access,
/// destructuring, iteration, and mutation.
mod compound_data_types;

/// Scalar data types: integers, floating-point numbers, booleans and characters.
mod scalar_data_types;

/// String data types: `&str`, `String` and string literals.
mod string_data_types;

/// The unit type `()` and the never type `!`.
mod unit_never;

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
    /// Runs the `compound_data_types` example.
    #[command(name = "compound_data_types")]
    CompoundDataTypes,

    /// Runs the `scalar_data_types` example.
    #[command(name = "scalar_data_types")]
    ScalarDataTypes,

    /// Runs the `string_data_types` example.
    #[command(name = "string_data_types")]
    StringDataTypes,

    /// Runs the `unit_never` example.
    #[command(name = "unit_never")]
    UnitNever,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::CompoundDataTypes => {
                compound_data_types::run();
            }
            Commands::ScalarDataTypes => {
                scalar_data_types::run();
            }
            Commands::StringDataTypes => {
                string_data_types::run();
            }
            Commands::UnitNever => {
                unit_never::run();
            }
        }
    }
}

/// Parses the command line and runs the selected example.
fn main() {
    dispatch(Cli::parse());
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &[
        "compound_data_types",
        "scalar_data_types",
        "string_data_types",
        "unit_never",
    ];

    /// Each example is reachable through its own subcommand.
    #[test]
    fn test_cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Verifies every subcommand parses.
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
