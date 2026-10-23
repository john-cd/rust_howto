//! Runs one of the `closures` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod closures;
mod closures_as_input_parameters;
mod closures_capture;
mod closures_move;
mod closures_with_type_annotations;

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
    /// Runs the `closures` example.
    #[command(name = "closures")]
    Closures,
    /// Runs the `closures_as_input_parameters` example.
    #[command(name = "closures_as_input_parameters")]
    ClosuresAsInputParameters,
    /// Runs the `closures_capture` example.
    #[command(name = "closures_capture")]
    ClosuresCapture,
    /// Runs the `closures_move` example.
    #[command(name = "closures_move")]
    ClosuresMove,
    /// Runs the `closures_with_type_annotations` example.
    #[command(name = "closures_with_type_annotations")]
    ClosuresWithTypeAnnotations,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Closures => {
                closures::run();
            }
            Commands::ClosuresAsInputParameters => {
                closures_as_input_parameters::run();
            }
            Commands::ClosuresCapture => {
                closures_capture::run();
            }
            Commands::ClosuresMove => {
                closures_move::run();
            }
            Commands::ClosuresWithTypeAnnotations => {
                closures_with_type_annotations::run();
            }
        }
    }
}
