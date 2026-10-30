//! Runs one of the `message_passing` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod flume;
mod flume_tick;
mod message_passing_crossbeam_channel;
mod message_passing_crossbeam_channel_after_tick;
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
