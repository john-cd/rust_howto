//! Configuration environment examples.

use clap::Parser;
use clap::Subcommand;

mod dotenvy;
mod env;
mod envy;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "dotenvy")]
    Dotenvy,
    #[command(name = "env")]
    Env,
    #[command(name = "envy")]
    Envy,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Dotenvy => {
                let _ = dotenvy::run();
            }
            Commands::Env => {
                let _ = env::run();
            }
            Commands::Envy => {
                let _ = envy::run();
            }
        }
    }
}
