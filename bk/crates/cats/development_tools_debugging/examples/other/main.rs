//! Runs one of the `other` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Log telemetry fields for an `OpenObserve` integration.
mod open_observe;
/// Create a manual OpenTelemetry span and pair it with regular tracing output.
mod open_telemetry;
/// This example demonstrates basic usage of the `slog` logging crate.
mod slog;
/// Propagate a W3C trace context from an HTTP client to a handler, so both
mod trace_context;
/// Printing the type of a value with `type_name_of_val`.
mod type_name_of_val;

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
    /// Runs the `open_observe` example.
    #[command(name = "open_observe")]
    OpenObserve,

    /// Runs the `open_telemetry` example.
    #[command(name = "open_telemetry")]
    OpenTelemetry,

    /// Runs the `slog` example.
    #[command(name = "slog")]
    Slog,

    /// Runs the `trace_context` example.
    #[command(name = "trace_context")]
    TraceContext,

    /// Runs the `type_name_of_val` example.
    #[command(name = "type_name_of_val")]
    TypeNameOfVal,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::OpenObserve => {
                open_observe::run()?;
            }
            Commands::OpenTelemetry => {
                open_telemetry::run()?;
            }
            Commands::Slog => {
                slog::run();
            }
            Commands::TraceContext => {
                trace_context::run()?;
            }
            Commands::TypeNameOfVal => {
                type_name_of_val::run();
            }
        }
    }
    Ok(())
}

/// Parses the command line and runs the selected example.
fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &[
        "open_observe",
        "open_telemetry",
        "slog",
        "trace_context",
        "type_name_of_val",
    ];

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
