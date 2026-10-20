use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

mod crux;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "crux")]
    Crux,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Crux => crux::run(),
        }
    }

    Ok(())
}
