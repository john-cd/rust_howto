use clap::Parser;
use clap::Subcommand;

mod streams;
mod streams2;
mod streams3;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "streams")]
    Streams,
    #[command(name = "streams2")]
    Streams2,
    #[command(name = "streams3")]
    Streams3,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Streams => {
                streams::run();
            }
            Commands::Streams2 => {
                streams2::run()?;
            }
            Commands::Streams3 => {
                streams3::run();
            }
        }
    }
    Ok(())
}
