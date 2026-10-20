use clap::Parser;
use clap::Subcommand;

mod actix;
mod actors;
mod ractor;
#[cfg(not(windows))]
mod riker;
#[cfg(not(windows))]
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
    #[cfg(not(windows))] // TODO review if this should be gated on a feature instead of the OS
    #[command(name = "riker")]
    Riker,
    #[cfg(not(windows))] // TODO review if this should be gated on a feature instead of the OS
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
            #[cfg(not(windows))]
            Commands::Riker => {
                riker::run();
            }
            #[cfg(not(windows))]
            Commands::Stakker => {
                stakker::run();
            }
        }
    }
}
