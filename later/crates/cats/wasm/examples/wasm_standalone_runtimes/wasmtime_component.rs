// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calls a typed WebAssembly *component* from Rust with `wasmtime`.
//!
//! The contract is the WIT package in `components/wit/greeter.wit`. The
//! `bindgen!` macro turns it into Rust types, a `Host` trait for the imports
//! the guest needs, and typed accessors for the exports. Unlike a core
//! `Module`, a component carries its interface types: strings, variants and
//! `result`s cross the boundary without manual pointer arithmetic, and a
//! component that does not match the world is rejected before it runs.
//!
//! The guest in `components/guest` is built with
//! `cargo build --release --target wasm32-wasip2`.

use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;
use wasmtime::Config;
use wasmtime::Engine;
use wasmtime::Store;
use wasmtime::StoreLimits;
use wasmtime::StoreLimitsBuilder;
use wasmtime::component::Component;
use wasmtime::component::HasSelf;
use wasmtime::component::Linker;
use wasmtime::component::ResourceTable;
use wasmtime::error::Context as _;
use wasmtime_wasi::WasiCtx;
use wasmtime_wasi::WasiCtxView;
use wasmtime_wasi::WasiView;

wasmtime::component::bindgen!({
    path: "examples/wasm_standalone_runtimes/components/wit",
    world: "greeter",
});

use exports::howto::greeter::greeting::GreetError;

/// Host state: what the guest's `logger` import writes to, plus limits.
struct State {
    /// Messages the guest logged.
    log: Vec<String>,
    /// Resource limits applied to the guest.
    limits: StoreLimits,
    /// The WASI configuration: empty, so the guest has no capabilities.
    wasi: WasiCtx,
    /// WASI resources (streams, ...) in use by the guest.
    table: ResourceTable,
}

impl WasiView for State {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

/// The host implementation of the `logger` import.
impl howto::greeter::logger::Host for State {
    fn log(&mut self, message: String) {
        self.log.push(message);
    }
}

/// Instantiates `component` with a fuel budget and a memory limit.
///
/// A component built for `wasm32-wasip2` imports a few WASI interfaces for
/// the Rust standard library. They are linked, but the `WasiCtx` is empty: no
/// preopened directories, environment variables, arguments, network or
/// inherited standard streams. Add capabilities with `WasiCtxBuilder` only
/// when the guest needs them.
fn instantiate(
    engine: &Engine,
    component: &Component,
) -> Result<(Store<State>, Greeter)> {
    let mut linker = Linker::new(engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    Greeter::add_to_linker::<State, HasSelf<State>>(&mut linker, |s| s)?;
    let mut store = Store::new(
        engine,
        State {
            log: Vec::new(),
            limits: StoreLimitsBuilder::new()
                .memory_size(16 * 1024 * 1024)
                .build(),
            wasi: WasiCtx::builder().build(),
            table: ResourceTable::new(),
        },
    );
    store.limiter(|state| &mut state.limits);
    store.set_fuel(10_000_000)?;
    let greeter = Greeter::instantiate(&mut store, component, &linker)
        .context("the component does not match the `greeter` world")?;
    Ok((store, greeter))
}

/// Builds the guest component and returns the path of the `.wasm` file.
///
/// Returns `None` when the `wasm32-wasip2` target is not installed
/// (`rustup target add wasm32-wasip2`).
fn build_guest() -> Result<Option<PathBuf>> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples/wasm_standalone_runtimes/components/guest");
    let target_dir = std::env::temp_dir().join("rust_howto_greeter_guest");
    let status = Command::new("cargo")
        .args(["build", "--release", "--target", "wasm32-wasip2"])
        .arg("--manifest-path")
        .arg(manifest_dir.join("Cargo.toml"))
        .arg("--target-dir")
        .arg(&target_dir)
        .env_remove("RUSTC_WRAPPER")
        .output()
        .context("failed to run cargo")?;
    if !status.status.success() {
        let stderr = String::from_utf8_lossy(&status.stderr);
        if stderr.contains("wasm32-wasip2")
            && stderr.contains("target may not be installed")
        {
            return Ok(None);
        }
        anyhow::bail!("building the guest failed:\n{stderr}");
    }
    Ok(Some(
        target_dir.join("wasm32-wasip2/release/greeter_guest.wasm"),
    ))
}

/// Builds the guest, then calls its exported `greet` function.
fn main() -> Result<()> {
    let Some(wasm) = build_guest()? else {
        println!("Skipping: run `rustup target add wasm32-wasip2` first.");
        return Ok(());
    };
    let mut config = Config::new();
    config.consume_fuel(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, wasm)?;
    let (mut store, greeter) = instantiate(&engine, &component)?;

    // A successful call returns the guest's `ok` value.
    let greeting = greeter
        .howto_greeter_greeting()
        .call_greet(&mut store, "Ferris")?
        .expect("a valid name is accepted");
    println!("{greeting}");
    assert_eq!(greeting, "Hello, Ferris!");

    // A `result::err` is a typed value, not a trap.
    let error = greeter
        .howto_greeter_greeting()
        .call_greet(&mut store, "")?
        .expect_err("an empty name is rejected");
    assert!(matches!(error, GreetError::EmptyName));
    let error = greeter
        .howto_greeter_greeting()
        .call_greet(&mut store, &"x".repeat(100))?
        .expect_err("a long name is rejected");
    assert!(matches!(error, GreetError::TooLong(32)));

    // The guest called back into the host through the `logger` import.
    assert_eq!(store.data().log.len(), 3);
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<()> {
    let entry = main;
    entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds the guest and returns an engine, or `None` without the target.
    fn engine_and_component() -> Result<Option<(Engine, Component)>> {
        let Some(wasm) = build_guest()? else {
            return Ok(None);
        };
        let mut config = Config::new();
        config.consume_fuel(true);
        let engine = Engine::new(&config)?;
        let component = Component::from_file(&engine, wasm)?;
        Ok(Some((engine, component)))
    }

    /// Runs the example through its entry point.
    #[test]
    fn example_succeeds() -> Result<()> {
        main()
    }

    /// Valid and invalid names produce the typed results.
    #[test]
    fn typed_results_cross_the_boundary() -> Result<()> {
        let Some((engine, component)) = engine_and_component()? else {
            return Ok(());
        };
        let (mut store, greeter) = instantiate(&engine, &component)?;
        let api = greeter.howto_greeter_greeting();
        assert_eq!(
            api.call_greet(&mut store, "Ada")?.unwrap_or_default(),
            "Hello, Ada!"
        );
        assert!(matches!(
            api.call_greet(&mut store, "")?,
            Err(GreetError::EmptyName)
        ));
        Ok(())
    }

    /// A component with the wrong shape is rejected at instantiation.
    #[test]
    fn contract_mismatch_is_rejected() -> Result<()> {
        let engine = Engine::default();
        let empty = Component::new(&engine, "(component)")?;
        assert!(instantiate(&engine, &empty).is_err());
        Ok(())
    }

    /// A core module is not a component.
    #[test]
    fn core_module_is_not_a_component() {
        let engine = Engine::default();
        assert!(Component::new(&engine, "(module)").is_err());
    }

    /// A guest that runs out of fuel traps instead of hanging the host.
    #[test]
    fn exhausted_fuel_traps() -> Result<()> {
        let Some((engine, component)) = engine_and_component()? else {
            return Ok(());
        };
        let (mut store, greeter) = instantiate(&engine, &component)?;
        store.set_fuel(10)?;
        let result = greeter
            .howto_greeter_greeting()
            .call_greet(&mut store, "Ada");
        assert!(result.is_err());
        Ok(())
    }
}
