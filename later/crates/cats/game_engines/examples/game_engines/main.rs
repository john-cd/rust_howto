// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "bevy")]
mod bevy;
#[cfg(feature = "fyrox")]
mod fyrox;
#[cfg(feature = "ggez")]
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
    #[cfg(feature = "bevy")]
    Bevy,
    #[command(name = "fyrox")]
    #[cfg(feature = "fyrox")]
    Fyrox,
    #[command(name = "ggez")]
    #[cfg(feature = "ggez")]
    Ggez,
    #[command(name = "macroquad")]
    Macroquad,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(feature = "bevy")]
            Commands::Bevy => {
                bevy::run();
            }
            #[cfg(feature = "fyrox")]
            Commands::Fyrox => {
                fyrox::run();
            }
            #[cfg(feature = "ggez")]
            Commands::Ggez => {
                ggez::run();
            }
            Commands::Macroquad => {
                macroquad::run();
            }
        }
    }
}
