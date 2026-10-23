//! Runs one of the `crossbeam` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod crossbeam_complex;
mod crossbeam_spawn;
mod crossbeam_spsc;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `crossbeam_complex` example.
    #[command(name = "crossbeam_complex")]
    CrossbeamComplex,
    /// Runs the `crossbeam_spawn` example.
    #[command(name = "crossbeam_spawn")]
    CrossbeamSpawn,
    /// Runs the `crossbeam_spsc` example.
    #[command(name = "crossbeam_spsc")]
    CrossbeamSpsc,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CrossbeamComplex => {
                crossbeam_complex::run();
            }
            Commands::CrossbeamSpawn => {
                crossbeam_spawn::run();
            }
            Commands::CrossbeamSpsc => {
                crossbeam_spsc::run();
            }
        }
    }
}
