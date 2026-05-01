use clap::{Parser, Subcommand};

mod cqrs;
mod di;
mod layered_architecture;
mod repository;
mod state_machine;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "cqrs")]
    Cqrs,
    #[command(name = "di")]
    Di,
    #[command(name = "layered_architecture")]
    LayeredArchitecture,
    #[command(name = "repository")]
    Repository,
    #[command(name = "state_machine")]
    StateMachine,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Cqrs => {
                let _ = cqrs::run();
            }
            Commands::Di => {
                let _ = di::run();
            }
            Commands::LayeredArchitecture => {
                let _ = layered_architecture::run();
            }
            Commands::Repository => {
                let _ = repository::run();
            }
            Commands::StateMachine => {
                let _ = state_machine::run();
            }
        }
    }
}
