//! Runs one of the `maps` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod indexmap;
mod linked_hash_map;
mod multimap;
mod slotmap;

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
    /// Runs the `indexmap` example.
    #[command(name = "indexmap")]
    Indexmap,
    /// Runs the `linked_hash_map` example.
    #[command(name = "linked_hash_map")]
    LinkedHashMap,
    /// Runs the `multimap` example.
    #[command(name = "multimap")]
    Multimap,
    /// Runs the `slotmap` example.
    #[command(name = "slotmap")]
    Slotmap,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Indexmap => {
                indexmap::run();
            }
            Commands::LinkedHashMap => {
                linked_hash_map::run();
            }
            Commands::Multimap => {
                multimap::run()?;
            }
            Commands::Slotmap => {
                slotmap::run();
            }
        }
    }
    Ok(())
}
