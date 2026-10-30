//! Runs one of the `control_flow` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod for1;
mod if_else;
mod labeled_block_expression;
mod loop1;
mod while1;

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
    /// Runs the `for1` example.
    #[command(name = "for1")]
    For1,
    /// Runs the `if_else` example.
    #[command(name = "if_else")]
    IfElse,
    /// Runs the `labeled_block_expression` example.
    #[command(name = "labeled_block_expression")]
    LabeledBlockExpression,
    /// Runs the `loop1` example.
    #[command(name = "loop1")]
    Loop1,
    /// Runs the `while1` example.
    #[command(name = "while1")]
    While1,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::For1 => {
                for1::run();
            }
            Commands::IfElse => {
                if_else::run();
            }
            Commands::LabeledBlockExpression => {
                labeled_block_expression::run();
            }
            Commands::Loop1 => {
                loop1::run();
            }
            Commands::While1 => {
                while1::run();
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
        "for1",
        "if_else",
        "labeled_block_expression",
        "loop1",
        "while1",
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
