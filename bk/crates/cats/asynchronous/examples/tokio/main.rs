//! Runs one of the `tokio` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Example of using a single-threaded Tokio runtime.
mod tokio2;
/// Demonstrates using `tokio::task::LocalSet` to run non-`Send` tasks.
mod tokio21;
/// Graceful shutdown of nested subsystems with `tokio_graceful_shutdown`.
mod tokio_graceful_shutdown;

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
    /// Runs the `tokio2` example.
    #[command(name = "tokio2")]
    Tokio2,
    /// Runs the `tokio21` example.
    #[command(name = "tokio21")]
    Tokio21,
    /// Runs the `tokio_graceful_shutdown` example.
    #[command(name = "tokio_graceful_shutdown")]
    TokioGracefulShutdown,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Tokio2 => {
                tokio2::run();
            }
            Commands::Tokio21 => {
                tokio21::run();
            }
            Commands::TokioGracefulShutdown => {
                tokio_graceful_shutdown::run()?;
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
    const SUBCOMMANDS: &[&str] = &["tokio2", "tokio21", "tokio_graceful_shutdown"];

    /// Confirms that the command and its arguments are valid.
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
}
