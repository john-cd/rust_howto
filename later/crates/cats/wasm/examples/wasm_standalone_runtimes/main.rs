//! Runs one of the `wasm_standalone_runtimes` examples, chosen on the command
//! line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod wasmer;
#[cfg(feature = "wasmtime")]
mod wasmtime;

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
    /// Runs the `wasmer` example.
    #[command(name = "wasmer")]
    Wasmer,
    /// Runs the `wasmtime` example.
    #[cfg(feature = "wasmtime")]
    #[command(name = "wasmtime")]
    Wasmtime,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Wasmer => {
                wasmer::run();
            }
            #[cfg(feature = "wasmtime")]
            Commands::Wasmtime => {
                wasmtime::run()?;
            }
        }
    }
    Ok(())
}
