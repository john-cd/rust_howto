use clap::Parser;
use clap::Subcommand;

mod async_compression;
mod flate2;
mod zip;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "async_compression")]
    AsyncCompression,
    #[command(name = "flate2")]
    Flate2,
    #[command(name = "zip")]
    Zip,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AsyncCompression => {
                async_compression::run();
            }
            Commands::Flate2 => {
                flate2::run();
            }
            Commands::Zip => {
                zip::run();
            }
        }
    }
}
