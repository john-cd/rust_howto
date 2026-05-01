use clap::{Parser, Subcommand};

mod dioxus;
mod tauri;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "dioxus")]
    Dioxus,
    #[command(name = "tauri")]
    Tauri,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Dioxus => {
                let _ = dioxus::run();
            }
            Commands::Tauri => {
                let _ = tauri::run();
            }
        }
    }
}
