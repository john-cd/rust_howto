// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod tokio2;
mod tokio21;
mod tokio_graceful_shutdown;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[derive(Subcommand)]
enum Commands {
    /// Runs the `tokio2` example.
    #[command(name = "tokio2")]
    Tokio2,
    /// Runs the `tokio21` example.
    #[command(name = "tokio21")]
    Tokio21,
    /// Runs the `tokio_graceful_shutdown` example.
    #[command(name = "tokio_graceful_shutdown")]
    TokioGracefulShutdown,
}

/// Runs the example selected on the command line.
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
