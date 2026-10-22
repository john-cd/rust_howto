// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod get;
mod get1;
mod header;

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
    /// Runs the `get` example.
    #[command(name = "get")]
    Get,
    /// Runs the `get1` example.
    #[command(name = "get1")]
    Get1,
    /// Runs the `header` example.
    #[command(name = "header")]
    Header,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Get => {
                get::run()?;
            }
            Commands::Get1 => {
                get1::run()?;
            }
            Commands::Header => {
                header::run()?;
            }
        }
    }

    Ok(())
}
