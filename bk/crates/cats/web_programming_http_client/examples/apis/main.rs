//! Runs one of the `apis` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod paginated;
mod rate_limited;
mod rest_get;
mod rest_head;
mod rest_post;

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
    /// Runs the `paginated` example.
    #[command(name = "paginated")]
    Paginated,
    /// Runs the `rate_limited` example.
    #[command(name = "rate_limited")]
    RateLimited,
    /// Runs the `rest_get` example.
    #[command(name = "rest_get")]
    RestGet,
    /// Runs the `rest_head` example.
    #[command(name = "rest_head")]
    RestHead,
    /// Runs the `rest_post` example.
    #[command(name = "rest_post")]
    RestPost,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Paginated => {
                paginated::run()?;
            }
            Commands::RateLimited => {
                rate_limited::run()?;
            }
            Commands::RestGet => {
                rest_get::run().map_err(|e| anyhow::anyhow!(e.to_string()))?;
            }
            Commands::RestHead => {
                rest_head::run()?;
            }
            Commands::RestPost => {
                rest_post::run()?;
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
    const SUBCOMMANDS: &[&str] = &["paginated", "rate_limited", "rest_get", "rest_head", "rest_post"];

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
