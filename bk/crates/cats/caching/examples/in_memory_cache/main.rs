//! Runs one of the `in_memory_cache` examples, chosen on the command line.
//!
//! Reviewed 10-27-2026.

use clap::Parser;
use clap::Subcommand;

mod cached;
mod lru;
mod moka;

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
    /// Runs the `cached` example.
    #[command(name = "cached")]
    Cached,
    /// Runs the `lru` example.
    #[command(name = "lru")]
    Lru,
    /// Runs the `moka` example.
    #[command(name = "moka")]
    Moka,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Cached => {
                cached::run();
            }
            Commands::Lru => {
                lru::run();
            }
            Commands::Moka => {
                moka::run();
            }
        }
    }
}
