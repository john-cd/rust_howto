use clap::Parser;
use clap::Subcommand;

mod central_tendency;
mod central_tendency1;
mod central_tendency2;
mod standard_deviation;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "central_tendency")]
    CentralTendency,
    #[command(name = "central_tendency1")]
    CentralTendency1,
    #[command(name = "central_tendency2")]
    CentralTendency2,
    #[command(name = "standard_deviation")]
    StandardDeviation,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CentralTendency => {
                let _ = central_tendency::run();
            }
            Commands::CentralTendency1 => {
                let _ = central_tendency1::run();
            }
            Commands::CentralTendency2 => {
                let _ = central_tendency2::run();
            }
            Commands::StandardDeviation => {
                let _ = standard_deviation::run();
            }
        }
    }
}
