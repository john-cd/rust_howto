use clap::Parser;
use clap::Subcommand;

mod eq;
mod ord;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "eq")]
    Eq,
    #[command(name = "ord")]
    Ord,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Eq => {
                let _ = eq::run();
            }
            Commands::Ord => {
                let _ = ord::run();
            }
        }
    }
}
