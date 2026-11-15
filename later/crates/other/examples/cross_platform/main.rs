//! Runs one of the `cross_platform` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

/// Implements the platform-independent counter core with Crux Core.
mod crux;

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
    /// Runs the `crux` example.
    #[command(name = "crux")]
    Crux,
}

/// Runs the example selected on the command line.
fn main() -> Result<()> {
    dispatch(Cli::parse())
}

/// Runs the example selected in the parsed command line.
fn dispatch(cli: Cli) -> Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Crux => crux::run(),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory as _;

    use super::*;

    /// Confirms that the command and its arguments are valid.
    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Every declared subcommand name is accepted.
    #[test]
    fn declared_subcommands_parse() {
        let command = Cli::command();
        let names: Vec<_> = command
            .get_subcommands()
            .map(clap::Command::get_name)
            .collect();
        for name in names {
            assert!(
                Cli::try_parse_from(["example", name]).is_ok(),
                "subcommand `{name}` should parse"
            );
        }
    }

    /// Unknown subcommands are rejected.
    #[test]
    fn unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }
}
