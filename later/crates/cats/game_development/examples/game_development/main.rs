use clap::{Parser, Subcommand};

mod game_development1;
mod glam;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "game_development1")]
    GameDevelopment1,
    #[command(name = "glam")]
    Glam,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::GameDevelopment1 => {
                game_development1::run();
            }
            Commands::Glam => {
                glam::run();
            }
        }
    }
}
