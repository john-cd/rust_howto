use clap::Parser;
use clap::Subcommand;

mod cors;
mod tower_http;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "cors")]
    Cors,
    #[command(name = "tower_http")]
    TowerHttp,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Cors => {
                cors::run()?;
            }
            Commands::TowerHttp => {
                tower_http::run();
            }
        }
    }
    Ok(())
}
