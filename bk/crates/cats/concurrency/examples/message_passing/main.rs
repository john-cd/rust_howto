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
                let _ = flume::run();
            }
            Commands::MessagePassingCrossbeamChannel => {
                let _ = message_passing_crossbeam_channel::run();
            }
            Commands::MessagePassingCrossbeamChannelAfterTick => {
                let _ = message_passing_crossbeam_channel_after_tick::run();
            }
            Commands::MessagePassingMpsc => {
                let _ = message_passing_mpsc::run();
            }
        }
    }
}
