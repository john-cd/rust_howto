// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod broken;
mod extract_links;
mod unique;

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
    /// Runs the `broken` example.
    #[command(name = "broken")]
    Broken,
    /// Runs the `extract_links` example.
    #[command(name = "extract_links")]
    ExtractLinks,
    /// Runs the `unique` example.
    #[command(name = "unique")]
    Unique,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Broken => {
                broken::run()?;
            }
            Commands::ExtractLinks => {
                extract_links::run()?;
            }
            Commands::Unique => {
                unique::run()?;
            }
        }
    }

    Ok(())
}
