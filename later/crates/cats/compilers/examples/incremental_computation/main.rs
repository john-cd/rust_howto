use clap::{Parser, Subcommand};

mod comemo;
mod salsa;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "comemo")]
    Comemo,
    #[command(name = "salsa")]
    Salsa,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Comemo => {
                let _ = comemo::run();
            }
            Commands::Salsa => {
                let _ = salsa::run();
            }
        }
    }
}
