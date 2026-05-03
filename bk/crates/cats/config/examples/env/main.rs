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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Dotenvy => {
                dotenvy::run()?;
            }
            Commands::Env => {
                env::run();
            }
            Commands::Envy => {
                envy::run();
            }
        }
    }

    Ok(())
}
