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
fn main() {
    let cli = Cli::parse();

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
                flume_tick::run();
            }
        }
    }
}
