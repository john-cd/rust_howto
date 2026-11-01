//! Runs a focused `crossbeam-channel` example from the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod crossbeam_complex;
mod crossbeam_select;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The Crossbeam channel examples.
#[derive(Subcommand)]
enum Commands {
    /// Runs a bounded multi-producer, multi-consumer pipeline.
    #[command(name = "bounded_mpmc")]
    BoundedMpmc,
    /// Selects messages from several channels.
    #[command(name = "select_channels")]
    SelectChannels,
}

/// Runs the selected channel example.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::BoundedMpmc => {
                crossbeam_complex::run()?;
            }
            Commands::SelectChannels => {
                crossbeam_select::run()?;
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
    const SUBCOMMANDS: &[&str] = &["bounded_mpmc", "select_channels"];

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
        dispatch(cli).expect("dispatching should succeed");
    }
}
