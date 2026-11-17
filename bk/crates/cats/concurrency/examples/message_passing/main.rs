//! Runs one of the `message_passing` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Channels from the `flume` crate: bounded and unbounded, sync and async, and
mod flume;
/// A bounded `flume` channel for commands sent to a loop that handles them
mod flume_tick;
/// Demonstrates the use of crossbeam channels for message passing between
mod message_passing_crossbeam_channel;

/// This example demonstrates the use of `crossbeam_channel`'s `tick` and `after` functions to
/// create channels that deliver messages periodically and after a certain duration, respectively.
mod message_passing_crossbeam_channel_after_tick;
/// This example demonstrates message passing using multiple producers and a single consumer.
mod message_passing_mpsc;

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
    /// Runs the `flume` example.
    #[command(name = "flume")]
    Flume,

    /// Runs the `message_passing_crossbeam_channel` example.
    #[command(name = "message_passing_crossbeam_channel")]
    MessagePassingCrossbeamChannel,

    /// Runs the `message_passing_crossbeam_channel_after_tick` example.
    #[command(name = "message_passing_crossbeam_channel_after_tick")]
    MessagePassingCrossbeamChannelAfterTick,

    /// Runs the `message_passing_mpsc` example.
    #[command(name = "message_passing_mpsc")]
    MessagePassingMpsc,

    /// Runs the `flume_tick` example.
    #[command(name = "flume_tick")]
    FlumeTick,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Flume => {
                flume::run();
            }
            Commands::MessagePassingCrossbeamChannel => {
                message_passing_crossbeam_channel::run();
            }
            Commands::MessagePassingCrossbeamChannelAfterTick => {
                message_passing_crossbeam_channel_after_tick::run();
            }
            Commands::MessagePassingMpsc => {
                message_passing_mpsc::run();
            }
            Commands::FlumeTick => {
                flume_tick::run()?;
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
        "flume",
        "message_passing_crossbeam_channel",
        "message_passing_crossbeam_channel_after_tick",
        "message_passing_mpsc",
        "flume_tick",
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
