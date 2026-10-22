//! Runs one of the `async_channels` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod async_channel;
mod async_channels_mpsc;
mod async_channels_oneshot;
mod async_channels_oneshot2;
mod kanal;
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
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

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
