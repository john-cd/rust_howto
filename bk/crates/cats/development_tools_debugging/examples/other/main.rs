//! Runs one of the `other` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod open_observe;
mod open_telemetry;
mod slog;
mod type_name_of_val;

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
    /// Runs the `open_observe` example.
    #[command(name = "open_observe")]
    OpenObserve,
    /// Runs the `open_telemetry` example.
    #[command(name = "open_telemetry")]
    OpenTelemetry,
    /// Runs the `slog` example.
    #[command(name = "slog")]
    Slog,
    /// Runs the `type_name_of_val` example.
    #[command(name = "type_name_of_val")]
    TypeNameOfVal,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::OpenObserve => {
                open_observe::run();
            }
            Commands::OpenTelemetry => {
                open_telemetry::run();
            }
            Commands::Slog => {
                slog::run();
            }
            Commands::TypeNameOfVal => {
                type_name_of_val::run();
            }
        }
    }
}
