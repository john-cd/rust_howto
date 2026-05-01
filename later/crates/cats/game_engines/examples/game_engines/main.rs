use clap::{Parser, Subcommand};

mod bevy;
mod fyrox;
mod ggez;
mod macroquad;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "bevy")]
    Bevy,
    #[command(name = "fyrox")]
    Fyrox,
    #[command(name = "ggez")]
    Ggez,
    #[command(name = "macroquad")]
    Macroquad,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Bevy => {
                let _ = bevy::run();
            }
            Commands::Fyrox => {
                let _ = fyrox::run();
            }
            Commands::Ggez => {
                let _ = ggez::run();
            }
            Commands::Macroquad => {
                let _ = macroquad::run();
            }
        }
    }
}
