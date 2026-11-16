//! Runs one of the `wasm_standalone_runtimes` examples, chosen on the command
//! line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Compiles inline WebAssembly text and calls an exported addition function with Wasmer.
#[cfg(feature = "wasmer")]
mod wasmer;

/// This example demonstrates how to use the `wasmtime` crate to run WebAssembly
#[cfg(feature = "wasmtime")]
mod wasmtime;

/// Calls a typed WebAssembly *component* from Rust with `wasmtime`.
#[cfg(feature = "wasmtime")]
mod wasmtime_component;

/// Run untrusted WebAssembly in a `wasmtime` sandbox: a fuel budget bounds the CPU time, a store
/// limit bounds memory, and the only capability the guest has is one host function (no WASI, no
/// filesystem, no network).
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

    /// Runs the `wasmtime_component` example.
    #[cfg(feature = "wasmtime")]
    #[command(name = "wasmtime_component")]
    WasmtimeComponent,

    /// Runs the `wasmtime_sandbox` example.
    #[cfg(feature = "wasmtime")]
    #[command(name = "wasmtime_sandbox")]
    WasmtimeSandbox,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}

/// Runs the example selected in the parsed command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
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
            Commands::WasmtimeComponent => {
                wasmtime_component::run()?;
            }
            #[cfg(feature = "wasmtime")]
            Commands::WasmtimeSandbox => {
                wasmtime_sandbox::run()?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory as _;

    use super::*;

    /// Confirms that the command and its arguments are valid.
    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Every declared subcommand name is accepted.
    #[test]
    fn declared_subcommands_parse() {
        let command = Cli::command();
        let names: Vec<_> = command
            .get_subcommands()
            .map(clap::Command::get_name)
            .collect();
        for name in names {
            assert!(
                Cli::try_parse_from(["example", name]).is_ok(),
                "subcommand `{name}` should parse"
            );
        }
    }

    /// Unknown subcommands are rejected.
    #[test]
    fn unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }
}
