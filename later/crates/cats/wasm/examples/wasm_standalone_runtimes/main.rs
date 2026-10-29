//! Runs one of the `wasm_standalone_runtimes` examples, chosen on the command
//! line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "wasmer")]
mod wasmer;
#[cfg(feature = "wasmtime")]
mod wasmtime;
#[cfg(feature = "wasmtime")]
mod wasmtime_sandbox;

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
    #[cfg(feature = "wasmer")]
    #[command(name = "wasmer")]
    Wasmer,
    /// Runs the `wasmtime` example.
    #[cfg(feature = "wasmtime")]
    #[command(name = "wasmtime")]
    Wasmtime,
    /// Runs the `wasmtime_sandbox` example.
    #[cfg(feature = "wasmtime")]
    #[command(name = "wasmtime_sandbox")]
    WasmtimeSandbox,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(feature = "wasmer")]
            Commands::Wasmer => {
                wasmer::run()?;
            }
            #[cfg(feature = "wasmtime")]
            Commands::Wasmtime => {
                wasmtime::run()?;
            }
            #[cfg(feature = "wasmtime")]
            Commands::WasmtimeSandbox => {
                wasmtime_sandbox::run()?;
            }
        }
    }
    Ok(())
}
