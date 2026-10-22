//! Documentation tools examples.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod rustdoc;
mod rustdoc2;
mod rustdoc3;

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
    /// Runs the `rustdoc` example.
    #[command(name = "rustdoc")]
    Rustdoc,
    /// Runs the `rustdoc2` example.
    #[command(name = "rustdoc2")]
    Rustdoc2,
    /// Runs the `rustdoc3` example.
    #[command(name = "rustdoc3")]
    Rustdoc3,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Rustdoc => {
                rustdoc::run();
            }
            Commands::Rustdoc2 => {
                rustdoc2::run();
            }
            Commands::Rustdoc3 => {
                rustdoc3::run();
            }
        }
    }
}
