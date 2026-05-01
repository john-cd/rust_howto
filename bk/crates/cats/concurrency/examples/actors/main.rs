use clap::Parser;
use clap::Subcommand;

mod actix;
mod actors;
mod ractor;
mod riker;
mod stakker;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "actix")]
    Actix,
    #[command(name = "actors")]
    Actors,
    #[command(name = "ractor")]
    Ractor,
    #[command(name = "riker")]
    Riker,
    #[command(name = "stakker")]
    Stakker,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Actix => {
                actix::run();
            }
            Commands::Actors => {
                actors::run();
            }
            Commands::Ractor => {
                ractor::run();
            }
            Commands::Riker => {
                riker::run();
            }
            Commands::Stakker => {
                stakker::run();
            }
        }
    }
}
