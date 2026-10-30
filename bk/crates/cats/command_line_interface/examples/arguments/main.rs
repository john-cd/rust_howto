//! Runs one of the `arguments` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use ::clap::Parser;
use ::clap::Subcommand;

mod clap_basic;
mod clap_complete;
#[path = "clap.rs"]
mod clap_example;
mod lexopt;
mod pico_args;

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
    /// Runs the `clap` example.
    #[command(name = "clap")]
    Clap,
    /// Runs the `clap_basic` example.
    #[command(name = "clap_basic")]
    ClapBasic,
    /// Runs the `clap_complete` example.
    #[command(name = "clap_complete")]
    ClapComplete,
    /// Runs the `lexopt` example.
    #[command(name = "lexopt")]
    Lexopt,
    /// Runs the `pico_args` example.
    #[command(name = "pico_args")]
    PicoArgs,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Clap => {
                clap_example::run()?;
            }
            Commands::ClapBasic => {
                clap_basic::run();
            }
            Commands::ClapComplete => {
                clap_complete::run()?;
            }
            Commands::Lexopt => {
                lexopt::run()?;
            }
            Commands::PicoArgs => {
                pico_args::run()?;
            }
        }
    }

    Ok(())
}

/// Parses the command line and runs the selected example.
fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] =
        &["clap", "clap_basic", "clap_complete", "lexopt", "pico_args"];

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
        dispatch(cli).expect("dispatching should succeed");
    }
}
