use clap::Parser;
use clap::Subcommand;

mod enums;
mod strum;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "enums")]
    Enums,
    #[command(name = "strum")]
    Strum,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Enums => {
                enums::run();
            }
            Commands::Strum => {
                strum::run();
            }
        }
    }
}
