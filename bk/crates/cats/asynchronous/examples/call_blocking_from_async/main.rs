//! Runs one of the `call_blocking_from_async` examples, chosen on the command
//! line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod call_blocking_from_async_rayon;
mod call_blocking_from_async_spawn_blocking;
mod call_blocking_from_async_spawn_dedicated_thread;

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
    /// Runs the `call_blocking_from_async_rayon` example.
    #[command(name = "call_blocking_from_async_rayon")]
    CallBlockingFromAsyncRayon,
    /// Runs the `call_blocking_from_async_spawn_blocking` example.
    #[command(name = "call_blocking_from_async_spawn_blocking")]
    CallBlockingFromAsyncSpawnBlocking,
    /// Runs the `call_blocking_from_async_spawn_dedicated_thread` example.
    #[command(name = "call_blocking_from_async_spawn_dedicated_thread")]
    CallBlockingFromAsyncSpawnDedicatedThread,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CallBlockingFromAsyncRayon => {
                call_blocking_from_async_rayon::run();
            }
            Commands::CallBlockingFromAsyncSpawnBlocking => {
                call_blocking_from_async_spawn_blocking::run();
            }
            Commands::CallBlockingFromAsyncSpawnDedicatedThread => {
                call_blocking_from_async_spawn_dedicated_thread::run();
            }
        }
    }
}
