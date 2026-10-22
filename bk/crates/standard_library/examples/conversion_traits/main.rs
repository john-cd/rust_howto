//! Runs one of the `conversion_traits` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod from;
mod from2;
mod from3;

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
    /// Runs the `from` example.
    #[command(name = "from")]
    From,
    /// Runs the `from2` example.
    #[command(name = "from2")]
    From2,
    /// Runs the `from3` example.
    #[command(name = "from3")]
    From3,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::From => {
                from::run();
            }
            Commands::From2 => {
                from2::run().map_err(|e| anyhow::anyhow!("{e:?}"))?;
            }
            Commands::From3 => {
                from3::run();
            }
        }
    }
    Ok(())
}
