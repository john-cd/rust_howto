//! Semver tools examples.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod semver_command;
mod semver_complex;
mod semver_latest;
mod semver_parse;
mod semver_prerelease;

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
    /// Runs the `semver_command` example.
    #[command(name = "semver_command")]
    Command,
    /// Runs the `semver_complex` example.
    #[command(name = "semver_complex")]
    Complex,
    /// Runs the `semver_latest` example.
    #[command(name = "semver_latest")]
    Latest,
    /// Runs the `semver_parse` example.
    #[command(name = "semver_parse")]
    Parse,
    /// Runs the `semver_prerelease` example.
    #[command(name = "semver_prerelease")]
    Prerelease,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Command => {
                semver_command::run()?;
            }
            Commands::Complex => {
                semver_complex::run()?;
            }
            Commands::Latest => {
                semver_latest::run()?;
            }
            Commands::Parse => {
                semver_parse::run()?;
            }
            Commands::Prerelease => {
                semver_prerelease::run()?;
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
    const SUBCOMMANDS: &[&str] = &["semver_command", "semver_complex", "semver_latest", "semver_parse", "semver_prerelease"];

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
