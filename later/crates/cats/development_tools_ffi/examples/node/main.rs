use clap::{Parser, Subcommand};

mod napi;
mod neon;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "napi")]
    Napi,
    #[command(name = "neon")]
    Neon,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Napi => {
                napi::run();
            }
            Commands::Neon => {
                neon::run();
            }
        }
    }
}
