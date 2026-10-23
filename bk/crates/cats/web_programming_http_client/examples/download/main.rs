//! Runs one of the `download` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod download;
mod partial;
mod post_file;

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
    /// Runs the `download` example.
    #[command(name = "download")]
    Download,
    /// Runs the `partial` example.
    #[command(name = "partial")]
    Partial,
    /// Runs the `post_file` example.
    #[command(name = "post_file")]
    PostFile,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Download => {
                download::run()?;
            }
            Commands::Partial => {
                partial::run()?;
            }
            Commands::PostFile => {
                post_file::run()?;
            }
        }
    }

    Ok(())
}
