use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "candle")]
mod candle;
mod linfa;
mod smartcore;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "candle")]
    #[command(name = "candle")]
    Candle,
    #[command(name = "linfa")]
    Linfa,
    #[command(name = "smartcore")]
    Smartcore,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(feature = "candle")]
            Commands::Candle => {
                candle::run();
            }
            Commands::Linfa => {
                linfa::run();
            }
            Commands::Smartcore => {
                smartcore::run();
            }
        }
    }
}
