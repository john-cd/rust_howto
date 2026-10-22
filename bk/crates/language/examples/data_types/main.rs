//! Runs one of the `data_types` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod compound_data_types;
mod scalar_data_types;
mod string_data_types;
mod unit_never;

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
    /// Runs the `compound_data_types` example.
    #[command(name = "compound_data_types")]
    CompoundDataTypes,
    /// Runs the `scalar_data_types` example.
    #[command(name = "scalar_data_types")]
    ScalarDataTypes,
    /// Runs the `string_data_types` example.
    #[command(name = "string_data_types")]
    StringDataTypes,
    /// Runs the `unit_never` example.
    #[command(name = "unit_never")]
    UnitNever,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CompoundDataTypes => {
                compound_data_types::run();
            }
            Commands::ScalarDataTypes => {
                scalar_data_types::run();
            }
            Commands::StringDataTypes => {
                string_data_types::run();
            }
            Commands::UnitNever => {
                unit_never::run();
            }
        }
    }
}
