use clap::Parser;
use clap::Subcommand;

mod borrow;
mod borrow2;
mod borrow3;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "borrow")]
    Borrow,
    #[command(name = "borrow2")]
    Borrow2,
    #[command(name = "borrow3")]
    Borrow3,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Borrow => {
                borrow::run();
            }
            Commands::Borrow2 => {
                borrow2::run();
            }
            Commands::Borrow3 => {
                borrow3::run();
            }
        }
    }
}
