//! Runs one of the `bitfield` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod bitflags;
mod bitvec;
mod fixedbitset;
mod flagset;
mod roaring;

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
    /// Runs the `bitflags` example.
    #[command(name = "bitflags")]
    Bitflags,
    /// Runs the `bitvec` example.
    #[command(name = "bitvec")]
    Bitvec,
    /// Runs the `flagset` example.
    #[command(name = "flagset")]
    Flagset,
    /// Runs the `roaring` example.
    #[command(name = "roaring")]
    Roaring,
    /// Runs the `fixedbitset` example.
    #[command(name = "fixedbitset")]
    Fixedbitset,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Bitflags => {
                bitflags::run();
            }
            Commands::Bitvec => {
                bitvec::run();
            }
            Commands::Flagset => {
                flagset::run();
            }
            Commands::Roaring => {
                roaring::run();
            }
            Commands::Fixedbitset => {
                fixedbitset::run();
            }
        }
    }
}
