//! Runs one of the `functions` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Functions that never return.
mod diverging_functions;
/// Passing and calling function pointers.
mod function_pointers;
/// Defining functions with arguments and return values.
mod functions;
/// Generic functions that take arguments of any type.
mod generic_functions;
/// Generic functions with trait bounds.
mod generic_functions2;
/// A generic function with a lifetime parameter.
mod generic_functions3;
/// Returning references and owned values from functions.
mod return_reference;

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
    /// Runs the `diverging_functions` example.
    #[command(name = "diverging_functions")]
    DivergingFunctions,
    /// Runs the `function_pointers` example.
    #[command(name = "function_pointers")]
    FunctionPointers,
    /// Runs the `functions` example.
    #[command(name = "functions")]
    Functions,
    /// Runs the `generic_functions` example.
    #[command(name = "generic_functions")]
    GenericFunctions,
    /// Runs the `generic_functions2` example.
    #[command(name = "generic_functions2")]
    GenericFunctions2,
    /// Runs the `generic_functions3` example.
    #[command(name = "generic_functions3")]
    GenericFunctions3,
    /// Runs the `return_reference` example.
    #[command(name = "return_reference")]
    ReturnReference,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::DivergingFunctions => {
                diverging_functions::run();
            }
            Commands::FunctionPointers => {
                function_pointers::run();
            }
            Commands::Functions => {
                functions::run();
            }
            Commands::GenericFunctions => {
                generic_functions::run();
            }
            Commands::GenericFunctions2 => {
                generic_functions2::run();
            }
            Commands::GenericFunctions3 => {
                generic_functions3::run();
            }
            Commands::ReturnReference => {
                return_reference::run();
            }
        }
    }
}
/// Runs the example.
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
        "diverging_functions",
        "function_pointers",
        "functions",
        "generic_functions",
        "generic_functions2",
        "generic_functions3",
        "return_reference",
    ];

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
