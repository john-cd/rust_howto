//! Runs one of the `ansi_term` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod ansi_term_basic;
mod ansi_term_basic1;
mod ansi_term_basic2;
mod colored;

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
    /// Runs the `ansi_term_basic` example.
    #[command(name = "ansi_term_basic")]
    AnsiTermBasic,
    /// Runs the `ansi_term_basic1` example.
    #[command(name = "ansi_term_basic1")]
    AnsiTermBasic1,
    /// Runs the `ansi_term_basic2` example.
    #[command(name = "ansi_term_basic2")]
    AnsiTermBasic2,
    /// Runs the `colored` example.
    #[command(name = "colored")]
    Colored,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::AnsiTermBasic => {
                ansi_term_basic::run();
            }
            Commands::AnsiTermBasic1 => {
                ansi_term_basic1::run();
            }
            Commands::AnsiTermBasic2 => {
                ansi_term_basic2::run();
            }
            Commands::Colored => {
                colored::run();
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
    const SUBCOMMANDS: &[&str] = &["ansi_term_basic", "ansi_term_basic1", "ansi_term_basic2", "colored"];

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
