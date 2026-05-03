use clap::Parser;
use clap::Subcommand;

mod openrr;
mod zenoh;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "openrr")]
    Openrr,
    #[command(name = "zenoh")]
    Zenoh,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Openrr => {
                openrr::run();
            }
            Commands::Zenoh => {
                zenoh::run()?;
            }
        }
    }
    Ok(())
}
