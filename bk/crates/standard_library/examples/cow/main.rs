//! Runs one of the `cow` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod cow_alternatives;
mod cow_as_function_param;
mod cow_to_borrowed_owned;
mod function_returning_cow;
mod into_cow;
mod modify_cow_in_place;

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
    /// Runs the `cow_alternatives` example.
    #[command(name = "cow_alternatives")]
    CowAlternatives,
    /// Runs the `cow_as_function_param` example.
    #[command(name = "cow_as_function_param")]
    CowAsFunctionParam,
    /// Runs the `cow_to_borrowed_owned` example.
    #[command(name = "cow_to_borrowed_owned")]
    CowToBorrowedOwned,
    /// Runs the `function_returning_cow` example.
    #[command(name = "function_returning_cow")]
    FunctionReturningCow,
    /// Runs the `into_cow` example.
    #[command(name = "into_cow")]
    IntoCow,
    /// Runs the `modify_cow_in_place` example.
    #[command(name = "modify_cow_in_place")]
    ModifyCowInPlace,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::CowAlternatives => {
                cow_alternatives::run();
            }
            Commands::CowAsFunctionParam => {
                cow_as_function_param::run();
            }
            Commands::CowToBorrowedOwned => {
                cow_to_borrowed_owned::run();
            }
            Commands::FunctionReturningCow => {
                function_returning_cow::run();
            }
            Commands::IntoCow => {
                into_cow::run();
            }
            Commands::ModifyCowInPlace => {
                modify_cow_in_place::run();
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
        "cow_alternatives",
        "cow_as_function_param",
        "cow_to_borrowed_owned",
        "function_returning_cow",
        "into_cow",
        "modify_cow_in_place",
    ];

    /// The command-line definition is internally consistent.
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

    /// Running without a subcommand selects nothing and succeeds.
    #[test]
    fn test_no_subcommand_runs_nothing() {
        let cli = Cli::try_parse_from(["example"])
            .expect("no subcommand is accepted");
        assert!(cli.command.is_none());
        dispatch(cli);
    }
}
