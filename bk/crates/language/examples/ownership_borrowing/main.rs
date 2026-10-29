//! Runs one of the `ownership_borrowing` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod borrowing_function;
mod borrowing_function_mutable;
mod borrowing_immutable;
mod clone;
mod copy;
mod move1;
mod move_function;
mod read_write_lock;
mod scope;

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
    /// Runs the `borrowing_function` example.
    #[command(name = "borrowing_function")]
    BorrowingFunction,
    /// Runs the `borrowing_function_mutable` example.
    #[command(name = "borrowing_function_mutable")]
    BorrowingFunctionMutable,
    /// Runs the `borrowing_immutable` example.
    #[command(name = "borrowing_immutable")]
    BorrowingImmutable,
    /// Runs the `clone` example.
    #[command(name = "clone")]
    Clone,
    /// Runs the `copy` example.
    #[command(name = "copy")]
    Copy,
    /// Runs the `move1` example.
    #[command(name = "move1")]
    Move1,
    /// Runs the `move_function` example.
    #[command(name = "move_function")]
    MoveFunction,
    /// Runs the `read_write_lock` example.
    #[command(name = "read_write_lock")]
    ReadWriteLock,
    /// Runs the `scope` example.
    #[command(name = "scope")]
    Scope,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::BorrowingFunction => {
                borrowing_function::run();
            }
            Commands::BorrowingFunctionMutable => {
                borrowing_function_mutable::run();
            }
            Commands::BorrowingImmutable => {
                borrowing_immutable::run();
            }
            Commands::Clone => {
                clone::run();
            }
            Commands::Copy => {
                copy::run();
            }
            Commands::Move1 => {
                move1::run();
            }
            Commands::MoveFunction => {
                move_function::run();
            }
            Commands::ReadWriteLock => {
                read_write_lock::run();
            }
            Commands::Scope => {
                scope::run();
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
    const SUBCOMMANDS: &[&str] = &["borrowing_function", "borrowing_function_mutable", "borrowing_immutable", "clone", "copy", "move1", "move_function", "read_write_lock", "scope"];

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
        let cli = Cli::try_parse_from(["example"]).expect("no subcommand is accepted");
        assert!(cli.command.is_none());
        dispatch(cli);
    }
}
