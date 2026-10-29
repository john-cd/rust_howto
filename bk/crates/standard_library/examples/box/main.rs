//! Runs one of the `box` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod box_basics;
mod box_deref;
mod box_dst;
mod box_recursive;
mod box_trait_objects;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `box_basics` example.
    #[command(name = "box_basics")]
    BoxBasics,
    /// Runs the `box_deref` example.
    #[command(name = "box_deref")]
    BoxDeref,
    /// Runs the `box_dst` example.
    #[command(name = "box_dst")]
    BoxDst,
    /// Runs the `box_recursive` example.
    #[command(name = "box_recursive")]
    BoxRecursive,
    /// Runs the `box_trait_objects` example.
    #[command(name = "box_trait_objects")]
    BoxTraitObjects,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::BoxBasics => {
                box_basics::run();
            }
            Commands::BoxDeref => {
                box_deref::run();
            }
            Commands::BoxDst => {
                box_dst::run();
            }
            Commands::BoxRecursive => {
                box_recursive::run();
            }
            Commands::BoxTraitObjects => {
                box_trait_objects::run();
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
    const SUBCOMMANDS: &[&str] = &["box_basics", "box_deref", "box_dst", "box_recursive", "box_trait_objects"];

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
