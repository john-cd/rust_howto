//! Runs one of the `text_layout` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod cosmic_text;
mod parley;

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
    /// Runs the `cosmic_text` example.
    #[command(name = "cosmic_text")]
    CosmicText,
    /// Runs the `parley` example.
    #[command(name = "parley")]
    Parley,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CosmicText => {
                cosmic_text::run();
            }
            Commands::Parley => {
                parley::run();
            }
        }
    }
}
