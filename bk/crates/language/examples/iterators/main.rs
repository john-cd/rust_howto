//! Runs one of the `iterators` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod consume_intoiterator;
mod implement_intoiterator;
mod iterator_adapters;
mod iterators;
mod return_iterator;
mod simple_iterators;

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
    /// Runs the `consume_intoiterator` example.
    #[command(name = "consume_intoiterator")]
    ConsumeIntoiterator,
    /// Runs the `implement_intoiterator` example.
    #[command(name = "implement_intoiterator")]
    ImplementIntoiterator,
    /// Runs the `iterator_adapters` example.
    #[command(name = "iterator_adapters")]
    IteratorAdapters,
    /// Runs the `iterators` example.
    #[command(name = "iterators")]
    Iterators,
    /// Runs the `return_iterator` example.
    #[command(name = "return_iterator")]
    ReturnIterator,
    /// Runs the `simple_iterators` example.
    #[command(name = "simple_iterators")]
    SimpleIterators,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::ConsumeIntoiterator => {
                consume_intoiterator::run();
            }
            Commands::ImplementIntoiterator => {
                implement_intoiterator::run();
            }
            Commands::IteratorAdapters => {
                iterator_adapters::run();
            }
            Commands::Iterators => {
                iterators::run();
            }
            Commands::ReturnIterator => {
                return_iterator::run();
            }
            Commands::SimpleIterators => {
                simple_iterators::run();
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
        "consume_intoiterator",
        "implement_intoiterator",
        "iterator_adapters",
        "iterators",
        "return_iterator",
        "simple_iterators",
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
