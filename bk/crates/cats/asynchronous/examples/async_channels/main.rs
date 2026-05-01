use clap::Parser;
use clap::Subcommand;

mod async_channel;
mod async_channels_mpsc;
mod async_channels_oneshot;
mod async_channels_oneshot2;
mod kanal;
mod postage;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "async_channel")]
    AsyncChannel,
    #[command(name = "async_channels_mpsc")]
    AsyncChannelsMpsc,
    #[command(name = "async_channels_oneshot")]
    AsyncChannelsOneshot,
    #[command(name = "async_channels_oneshot2")]
    AsyncChannelsOneshot2,
    #[command(name = "kanal")]
    Kanal,
    #[command(name = "postage")]
    Postage,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AsyncChannel => {
                let _ = async_channel::run();
            }
            Commands::AsyncChannelsMpsc => {
                let _ = async_channels_mpsc::run();
            }
            Commands::AsyncChannelsOneshot => {
                let _ = async_channels_oneshot::run();
            }
            Commands::AsyncChannelsOneshot2 => {
                let _ = async_channels_oneshot2::run();
            }
            Commands::Kanal => {
                let _ = kanal::run();
            }
            Commands::Postage => {
                let _ = postage::run();
            }
        }
    }
}
