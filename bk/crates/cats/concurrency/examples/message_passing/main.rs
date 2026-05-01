use clap::Parser;
use clap::Subcommand;

mod flume;
mod message_passing_crossbeam_channel;
mod message_passing_crossbeam_channel_after_tick;
mod message_passing_mpsc;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "flume")]
    Flume,
    #[command(name = "message_passing_crossbeam_channel")]
    MessagePassingCrossbeamChannel,
    #[command(name = "message_passing_crossbeam_channel_after_tick")]
    MessagePassingCrossbeamChannelAfterTick,
    #[command(name = "message_passing_mpsc")]
    MessagePassingMpsc,
}

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
        }
    }
}
