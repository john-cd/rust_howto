use clap::Parser;
use clap::Subcommand;

mod futures1;
mod futures2;
mod futures3;
mod futures_util;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "futures1")]
    Futures1,
    #[command(name = "futures2")]
    Futures2,
    #[command(name = "futures3")]
    Futures3,
    #[command(name = "futures_util")]
    FuturesUtil,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Futures1 => {
                let _ = futures1::run();
            }
            Commands::Futures2 => {
                let _ = futures2::run();
            }
            Commands::Futures3 => {
                let _ = futures3::run();
            }
            Commands::FuturesUtil => {
                let _ = futures_util::run();
            }
        }
    }
}
