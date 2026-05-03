use clap::Parser;
use clap::Subcommand;

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
                bevy::run();
            }
            Commands::Fyrox => {
                fyrox::run();
            }
            Commands::Ggez => {
                ggez::run();
            }
            Commands::Macroquad => {
                macroquad::run();
            }
        }
    }
}
