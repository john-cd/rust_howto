// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "dioxus")]
mod dioxus;
#[cfg(all(not(windows), feature = "tauri"))] // TODO review Windows support
mod tauri;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "dioxus")]
    #[cfg(feature = "dioxus")]
    Dioxus,
    #[cfg(all(not(windows), feature = "tauri"))]
    #[command(name = "tauri")]
    Tauri,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(feature = "dioxus")]
            Commands::Dioxus => {
                dioxus::run();
            }
            #[cfg(all(not(windows), feature = "tauri"))]
            Commands::Tauri => {
                tauri::run();
            }
        }
    }
}
