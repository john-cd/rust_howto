use clap::Parser;
use clap::Subcommand;

mod json;
mod monostate;
mod serde;
mod serde_ignored;
mod serde_json;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "json")]
    Json,
    #[command(name = "monostate")]
    Monostate,
    #[command(name = "serde")]
    Serde,
    #[command(name = "serde_ignored")]
    SerdeIgnored,
    #[command(name = "serde_json")]
    SerdeJson,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Json => {
                let _ = json::run();
            }
            Commands::Monostate => {
                let _ = monostate::run();
            }
            Commands::Serde => {
                let _ = serde::run();
            }
            Commands::SerdeIgnored => {
                let _ = serde_ignored::run();
            }
            Commands::SerdeJson => {
                let _ = serde_json::run();
            }
        }
    }
}
