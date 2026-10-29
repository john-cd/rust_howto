//! Runs one of the `option` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod option_combinators;
mod option_match;
mod option_question_mark;
mod option_ref;
mod option_unwrap;
mod options1;
mod options2;

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
    /// Runs the `option_combinators` example.
    #[command(name = "option_combinators")]
    OptionCombinators,
    /// Runs the `option_match` example.
    #[command(name = "option_match")]
    OptionMatch,
    /// Runs the `option_question_mark` example.
    #[command(name = "option_question_mark")]
    OptionQuestionMark,
    /// Runs the `option_ref` example.
    #[command(name = "option_ref")]
    OptionRef,
    /// Runs the `option_unwrap` example.
    #[command(name = "option_unwrap")]
    OptionUnwrap,
    /// Runs the `options1` example.
    #[command(name = "options1")]
    Options1,
    /// Runs the `options2` example.
    #[command(name = "options2")]
    Options2,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::OptionCombinators => {
                option_combinators::run();
            }
            Commands::OptionMatch => {
                option_match::run();
            }
            Commands::OptionQuestionMark => {
                option_question_mark::run();
            }
            Commands::OptionRef => {
                option_ref::run();
            }
            Commands::OptionUnwrap => {
                option_unwrap::run();
            }
            Commands::Options1 => {
                options1::run();
            }
            Commands::Options2 => {
                options2::run();
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
    const SUBCOMMANDS: &[&str] = &["option_combinators", "option_match", "option_question_mark", "option_ref", "option_unwrap", "options1", "options2"];

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
