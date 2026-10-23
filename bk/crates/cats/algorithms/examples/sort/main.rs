//! Runs one of the `sort` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod sort_basic;
mod sort_float;
mod sort_struct;

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
    /// Runs the `sort_basic` example.
    #[command(name = "sort_basic")]
    Basic,
    /// Runs the `sort_float` example.
    #[command(name = "sort_float")]
    Float,
    /// Runs the `sort_struct` example.
    #[command(name = "sort_struct")]
    Struct,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Basic => {
                sort_basic::run();
            }
            Commands::Float => {
                sort_float::run();
            }
            Commands::Struct => {
                sort_struct::run();
            }
        }
    }
}
