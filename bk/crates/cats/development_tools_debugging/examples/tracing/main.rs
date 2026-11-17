//! Runs one of the `tracing` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// This example demonstrates the basic usage of the `tracing` crate for logging
mod tracing;
/// Instrumenting sync and async functions with `tracing` spans.
mod tracing_instrument;
/// Demonstrates entering and exiting a span using `entered()` and `exit()`.
mod tracing_span_entered;
/// Demonstrates how to use tracing spans to instrument code,
mod tracing_span_in_scope;
/// Creating and entering spans with `tracing`.
mod tracing_spans;

/// - `tracing` is a framework for instrumenting Rust programs to collect scoped, structured, and
/// async-aware diagnostics.
mod tracing_subscriber;
/// Demonstrates how to configure the tracing subscriber
mod tracing_subscriber1;
/// Demonstrates how to log in JSON format.
mod tracing_subscriber1b;
/// Demonstrates how to write logs to multiple destinations (in this instance,
mod tracing_subscriber2b;
/// This example demonstrates how to enable or
mod tracing_subscriber3;

/// Demonstrates how to configure tracing at runtime, - dealing with a variable number of `Layer`s
/// of different types.
mod tracing_subscriber3b;
/// This example demonstrates customizing the formatting of log lines for spans and events.
mod tracing_subscriber4;
/// Composing a `tracing` subscriber from layers and filters.
mod tracing_subscriber_layers;

#[cfg(test)]
/// This module provides the `isolated` implementation used by its parent.
#[path = "../support/isolated.rs"]
mod isolated;

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
    /// Runs the `tracing` example.
    #[command(name = "tracing")]
    Tracing,

    /// Runs the `tracing_instrument` example.
    #[command(name = "tracing_instrument")]
    TracingInstrument,

    /// Runs the `tracing_span_entered` example.
    #[command(name = "tracing_span_entered")]
    TracingSpanEntered,

    /// Runs the `tracing_span_in_scope` example.
    #[command(name = "tracing_span_in_scope")]
    TracingSpanInScope,

    /// Runs the `tracing_spans` example.
    #[command(name = "tracing_spans")]
    TracingSpans,

    /// Runs the `tracing_subscriber` example.
    #[command(name = "tracing_subscriber")]
    TracingSubscriber,

    /// Runs the `tracing_subscriber1` example.
    #[command(name = "tracing_subscriber1")]
    TracingSubscriber1,

    /// Runs the `tracing_subscriber1b` example.
    #[command(name = "tracing_subscriber1b")]
    TracingSubscriber1b,

    /// Runs the example that composes tracing subscriber layers.
    #[command(name = "tracing_subscriber_layers", alias = "tracing_subscriber2")]
    TracingSubscriberLayers,

    /// Runs the `tracing_subscriber2b` example.
    #[command(name = "tracing_subscriber2b")]
    TracingSubscriber2b,

    /// Runs the `tracing_subscriber3` example.
    #[command(name = "tracing_subscriber3")]
    TracingSubscriber3,

    /// Runs the `tracing_subscriber3b` example.
    #[command(name = "tracing_subscriber3b")]
    TracingSubscriber3b,

    /// Runs the `tracing_subscriber4` example.
    #[command(name = "tracing_subscriber4")]
    TracingSubscriber4,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Tracing => {
                tracing::run();
            }
            Commands::TracingInstrument => {
                tracing_instrument::run();
            }
            Commands::TracingSpanEntered => {
                tracing_span_entered::run();
            }
            Commands::TracingSpanInScope => {
                tracing_span_in_scope::run();
            }
            Commands::TracingSpans => {
                tracing_spans::run();
            }
            Commands::TracingSubscriber => {
                tracing_subscriber::run();
            }
            Commands::TracingSubscriber1 => {
                tracing_subscriber1::run();
            }
            Commands::TracingSubscriber1b => {
                tracing_subscriber1b::run();
            }
            Commands::TracingSubscriberLayers => {
                tracing_subscriber_layers::run().map_err(|e| anyhow::anyhow!(e.to_string()))?;
            }
            Commands::TracingSubscriber2b => {
                tracing_subscriber2b::run();
            }
            Commands::TracingSubscriber3 => {
                tracing_subscriber3::run()?;
            }
            Commands::TracingSubscriber3b => {
                tracing_subscriber3b::run()?;
            }
            Commands::TracingSubscriber4 => {
                tracing_subscriber4::run();
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
        "tracing",
        "tracing_instrument",
        "tracing_span_entered",
        "tracing_span_in_scope",
        "tracing_spans",
        "tracing_subscriber",
        "tracing_subscriber1",
        "tracing_subscriber1b",
        "tracing_subscriber2b",
        "tracing_subscriber3",
        "tracing_subscriber3b",
        "tracing_subscriber4",
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
