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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Json => {
                json::run()?;
            }
            Commands::Monostate => {
                monostate::run();
            }
            Commands::Serde => {
                serde::run();
            }
            Commands::SerdeIgnored => {
                serde_ignored::run();
            }
            Commands::SerdeJson => {
                serde_json::run();
            }
        }
    }
    Ok(())
}
