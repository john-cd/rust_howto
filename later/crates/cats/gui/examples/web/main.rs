use clap::Parser;
use clap::Subcommand;

mod dioxus;
#[cfg(not(windows))] // TODO review Windows support
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
    #[cfg(not(windows))]
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
            #[cfg(not(windows))]
            Commands::Tauri => {
                tauri::run();
            }
        }
    }
}
