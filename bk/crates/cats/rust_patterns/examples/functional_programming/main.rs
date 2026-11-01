//! Runs one of the `functional_programming` examples, chosen on the command
//! line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod either;
mod frunk;
mod imbl;
mod iterators;
mod itertools;
mod rpds;

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
    /// Runs the `either` example.
    #[command(name = "either")]
    Either,
    /// Runs the `frunk` example.
    #[command(name = "frunk")]
    Frunk,
    /// Runs the `imbl` example.
    #[command(name = "imbl")]
    Imbl,
    /// Runs the `itertools` example.
    #[command(name = "itertools")]
    Itertools,
    /// Runs the standard-library iterator adapter example.
    #[command(name = "iterators")]
    Iterators,
    /// Runs the `rpds` example.
    #[command(name = "rpds")]
    Rpds,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::Either => {
                either::run();
            }
            Commands::Frunk => {
                frunk::run();
            }
            Commands::Imbl => {
                imbl::run();
            }
            Commands::Itertools => {
                itertools::run();
            }
            Commands::Iterators => {
                iterators::run();
            }
            Commands::Rpds => {
                rpds::run();
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
    const SUBCOMMANDS: &[&str] = &["either", "frunk", "imbl", "itertools", "iterators", "rpds"];

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
