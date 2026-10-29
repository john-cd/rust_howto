//! Runs one of the `actors` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod actix;
mod actors;
mod ractor;
// Disabled with the `riker` Cargo feature while its dependency chain is
// vulnerable.

// #[cfg(feature = "riker")]
// mod riker;
#[cfg(feature = "stakker")]
mod stakker;

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
    /// Runs the `actix` example.
    #[command(name = "actix")]
    Actix,
    /// Runs the `actors` example.
    #[command(name = "actors")]
    Actors,
    /// Runs the `ractor` example.
    #[command(name = "ractor")]
    Ractor,
    // Disabled with the `riker` Cargo feature while its dependency chain is
    // vulnerable.

    // /// Runs the `riker` example.
    // #[cfg(feature = "riker")]
    // #[command(name = "riker")]
    // Riker,
    /// Runs the `stakker` example.
    #[cfg(feature = "stakker")]
    #[command(name = "stakker")]
    Stakker,
}

/// Runs the example selected on the command line.
///
/// # Errors
/// Returns an error if the selected actor example cannot complete its message
/// exchange.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Actix => {
                actix::run()?;
            }
            Commands::Actors => {
                actors::run()?;
            }
            Commands::Ractor => {
                ractor::run()?;
            }
            // Disabled with the `riker` Cargo feature while its dependency
            // chain is vulnerable.

            // #[cfg(feature = "riker")]
            // Commands::Riker => {
            //     riker::run()?;
            // }
            #[cfg(feature = "stakker")]
            Commands::Stakker => {
                stakker::run()?;
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
    const SUBCOMMANDS: &[&str] = &["actix", "actors", "ractor", "stakker"];

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
