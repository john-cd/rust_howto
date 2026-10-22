//! Runs one of the `slices` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod slice_as_argument;
mod slices;
mod string_slices;

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
    /// Runs the `slice_as_argument` example.
    #[command(name = "slice_as_argument")]
    SliceAsArgument,
    /// Runs the `slices` example.
    #[command(name = "slices")]
    Slices,
    /// Runs the `string_slices` example.
    #[command(name = "string_slices")]
    StringSlices,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::SliceAsArgument => {
                slice_as_argument::run();
            }
            Commands::Slices => {
                slices::run();
            }
            Commands::StringSlices => {
                string_slices::run();
            }
        }
    }
}
