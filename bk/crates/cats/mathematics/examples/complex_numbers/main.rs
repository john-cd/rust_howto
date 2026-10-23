//! Runs one of the `complex_numbers` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod add_complex;
mod create_complex;
mod mathematical_functions;

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
    /// Runs the `add_complex` example.
    #[command(name = "add_complex")]
    AddComplex,
    /// Runs the `create_complex` example.
    #[command(name = "create_complex")]
    CreateComplex,
    /// Runs the `mathematical_functions` example.
    #[command(name = "mathematical_functions")]
    MathematicalFunctions,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AddComplex => {
                add_complex::run();
            }
            Commands::CreateComplex => {
                create_complex::run();
            }
            Commands::MathematicalFunctions => {
                mathematical_functions::run();
            }
        }
    }
}
