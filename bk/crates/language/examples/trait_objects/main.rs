//! Runs one of the `trait_objects` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod dyn_autotraits;
mod dyn_compat;
mod dyn_supertraits;
mod trait_objects;

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
    /// Runs the `dyn_autotraits` example.
    #[command(name = "dyn_autotraits")]
    DynAutotraits,
    /// Runs the `dyn_compat` example.
    #[command(name = "dyn_compat")]
    DynCompat,
    /// Runs the `dyn_supertraits` example.
    #[command(name = "dyn_supertraits")]
    DynSupertraits,
    /// Runs the `trait_objects` example.
    #[command(name = "trait_objects")]
    TraitObjects,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::DynAutotraits => {
                dyn_autotraits::run();
            }
            Commands::DynCompat => {
                dyn_compat::run();
            }
            Commands::DynSupertraits => {
                dyn_supertraits::run();
            }
            Commands::TraitObjects => {
                trait_objects::run();
            }
        }
    }
}
