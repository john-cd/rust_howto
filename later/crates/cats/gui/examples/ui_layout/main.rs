use clap::{Parser, Subcommand};

mod morphorm;
mod taffy;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "morphorm")]
    Morphorm,
    #[command(name = "taffy")]
    Taffy,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Morphorm => {
                morphorm::run();
            }
            Commands::Taffy => {
                taffy::run();
            }
        }
    }
}
