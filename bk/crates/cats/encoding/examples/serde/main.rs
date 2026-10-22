// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod json;
mod monostate;
mod serde;
mod serde_ignored;
mod serde_json;

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
    /// Runs the `json` example.
    #[command(name = "json")]
    Json,
    /// Runs the `monostate` example.
    #[command(name = "monostate")]
    Monostate,
    /// Runs the `serde` example.
    #[command(name = "serde")]
    Serde,
    /// Runs the `serde_ignored` example.
    #[command(name = "serde_ignored")]
    SerdeIgnored,
    /// Runs the `serde_json` example.
    #[command(name = "serde_json")]
    SerdeJson,
}

/// Runs the example selected on the command line.
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
