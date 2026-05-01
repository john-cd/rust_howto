use clap::Parser;
use clap::Subcommand;

mod winapi;
mod windows;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "winapi")]
    Winapi,
    #[command(name = "windows")]
    Windows,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Winapi => {
                let _ = winapi::run();
            }
            Commands::Windows => {
                let _ = windows::run();
            }
        }
    }
}
