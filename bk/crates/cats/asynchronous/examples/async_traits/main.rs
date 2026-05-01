use clap::Parser;
use clap::Subcommand;

mod async_traits;
mod async_traits2;
mod async_traits3;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "async_traits")]
    AsyncTraits,
    #[command(name = "async_traits2")]
    AsyncTraits2,
    #[command(name = "async_traits3")]
    AsyncTraits3,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AsyncTraits => {
                let _ = async_traits::run();
            }
            Commands::AsyncTraits2 => {
                let _ = async_traits2::run();
            }
            Commands::AsyncTraits3 => {
                let _ = async_traits3::run();
            }
        }
    }
}
