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
                let _ = morphorm::run();
            }
            Commands::Taffy => {
                let _ = taffy::run();
            }
        }
    }
}
