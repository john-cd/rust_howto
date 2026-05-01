use clap::Parser;
use clap::Subcommand;

mod deadpool;
#[cfg(feature = "postgres")]
mod deadpool2;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "deadpool")]
    Deadpool,
    #[cfg(feature = "postgres")]
    #[command(name = "deadpool2")]
    Deadpool2,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Deadpool => {
                let _ = deadpool::run();
            }
            #[cfg(feature = "postgres")]
            Commands::Deadpool2 => {
                let _ = deadpool2::run();
            }
        }
    }
}
