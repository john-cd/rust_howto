use clap::Parser;
use clap::Subcommand;

mod latitude_longitude;
mod side_length;
mod tan_sin_cos;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "latitude_longitude")]
    LatitudeLongitude,
    #[command(name = "side_length")]
    SideLength,
    #[command(name = "tan_sin_cos")]
    TanSinCos,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::LatitudeLongitude => {
                latitude_longitude::run();
            }
            Commands::SideLength => {
                side_length::run();
            }
            Commands::TanSinCos => {
                tan_sin_cos::run();
            }
        }
    }
}
