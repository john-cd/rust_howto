//! Runs one of the `async_channels` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Multi-producer, multi-consumer (MPMC) async channel communication.
mod async_channel;

/// A Tokio multi-producer, single-consumer (`mpsc`) channel.
mod async_channels_mpsc;

/// A Tokio `oneshot` channel that carries a single value between tasks.
mod async_channels_oneshot;

/// This example demonstrates the use of `tokio::sync::oneshot` channels to communicate between
/// asynchronous tasks.
mod async_channels_oneshot2;

/// The `kanal` crate is used for inter-thread communication, similar to Rust's standard
/// `std::sync::mpsc` module but with additional features and capabilities.
mod kanal;

/// A broadcast channel from the `postage` crate: several receivers get every
mod postage;

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
    /// Runs the `async_channel` example.
    #[command(name = "async_channel")]
    AsyncChannel,

    /// Runs the `async_channels_mpsc` example.
    #[command(name = "async_channels_mpsc")]
    AsyncChannelsMpsc,

    /// Runs the `async_channels_oneshot` example.
    #[command(name = "async_channels_oneshot")]
    AsyncChannelsOneshot,

    /// Runs the `async_channels_oneshot2` example.
    #[command(name = "async_channels_oneshot2")]
    AsyncChannelsOneshot2,

    /// Runs the `kanal` example.
    #[command(name = "kanal")]
    Kanal,

    /// Runs the `postage` example.
    #[command(name = "postage")]
    Postage,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::AsyncChannel => {
                async_channel::run();
            }
            Commands::AsyncChannelsMpsc => {
                async_channels_mpsc::run();
            }
            Commands::AsyncChannelsOneshot => {
                async_channels_oneshot::run();
            }
            Commands::AsyncChannelsOneshot2 => {
                async_channels_oneshot2::run();
            }
            Commands::Kanal => {
                kanal::run()?;
            }
            Commands::Postage => {
                postage::run();
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
        "async_channel",
        "async_channels_mpsc",
        "async_channels_oneshot",
        "async_channels_oneshot2",
        "kanal",
        "postage",
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
