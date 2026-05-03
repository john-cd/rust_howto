use clap::Parser;
use clap::Subcommand;

mod tokio2;
mod tokio21;
mod tokio_graceful_shutdown;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "tokio2")]
    Tokio2,
    #[command(name = "tokio21")]
    Tokio21,
    #[command(name = "tokio_graceful_shutdown")]
    TokioGracefulShutdown,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Tokio2 => {
                tokio2::run();
            }
            Commands::Tokio21 => {
                tokio21::run();
            }
            Commands::TokioGracefulShutdown => {
                tokio_graceful_shutdown::run()?;
            }
        }
    }
    Ok(())
}
