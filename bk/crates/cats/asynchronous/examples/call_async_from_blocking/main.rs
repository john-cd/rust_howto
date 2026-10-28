//! Runs one of the `call_async_from_blocking` examples, chosen on the command
//! line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod call_async_from_blocking_futures_executor;
mod call_async_from_blocking_tokio_runtime;

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
    /// Runs the `call_async_from_blocking_futures_executor` example.
    #[command(name = "call_async_from_blocking_futures_executor")]
    CallAsyncFromBlockingFuturesExecutor,
    /// Runs the `call_async_from_blocking_tokio_runtime` example.
    #[command(name = "call_async_from_blocking_tokio_runtime")]
    CallAsyncFromBlockingTokioRuntime,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CallAsyncFromBlockingFuturesExecutor => {
                call_async_from_blocking_futures_executor::run();
            }
            Commands::CallAsyncFromBlockingTokioRuntime => {
                call_async_from_blocking_tokio_runtime::run();
            }
        }
    }
}
