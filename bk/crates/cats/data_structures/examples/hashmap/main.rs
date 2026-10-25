//! Runs one of the `hashmap` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod custom_hash_function;
mod custom_type_as_key;
mod fast_hashmap;
mod hashmap;
mod hashset;

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
    /// Runs the `custom_hash_function` example.
    #[command(name = "custom_hash_function")]
    CustomHashFunction,
    /// Runs the `custom_type_as_key` example.
    #[command(name = "custom_type_as_key")]
    CustomTypeAsKey,
    /// Runs the `hashmap` example.
    #[command(name = "hashmap")]
    Hashmap,
    /// Runs the `hashset` example.
    #[command(name = "hashset")]
    Hashset,
    /// Runs the `fast_hashmap` example.
    #[command(name = "fast_hashmap")]
    FastHashmap,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CustomHashFunction => {
                custom_hash_function::run();
            }
            Commands::CustomTypeAsKey => {
                custom_type_as_key::run();
            }
            Commands::Hashmap => {
                hashmap::run();
            }
            Commands::Hashset => {
                hashset::run();
            }
            Commands::FastHashmap => {
                fast_hashmap::run();
            }
        }
    }
}
