//! Runs one of the `c` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod bindgen;
mod cbindgen;

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
    /// Runs the `bindgen` example.
    #[command(name = "bindgen")]
    Bindgen,
    /// Runs the `cbindgen` example.
    #[command(name = "cbindgen")]
    Cbindgen,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Bindgen => {
                bindgen::run();
            }
            Commands::Cbindgen => {
                cbindgen::run();
            }
        }
    }
}
