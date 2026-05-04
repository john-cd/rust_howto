use clap::Parser;
use clap::Subcommand;

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
    Dioxus,
    #[cfg(all(not(windows), feature = "tauri"))]
    #[command(name = "tauri")]
    Tauri,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
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
