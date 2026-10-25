#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Run untrusted WebAssembly in a `wasmtime` sandbox: a fuel budget bounds the
//! CPU time, a store limit bounds memory, and the only capability the guest
//! has is one host function (no WASI, no filesystem, no network).

use std::collections::HashMap;

use anyhow::Result;
use wasmtime::Config;
use wasmtime::Engine;
use wasmtime::Linker;
use wasmtime::Module;
use wasmtime::Store;
use wasmtime::StoreLimits;
use wasmtime::StoreLimitsBuilder;
use wasmtime::Trap;

/// Guest code: `run` calls the host's `double`, `spin` never returns,
/// `grow` tries to grow the linear memory by `n` pages.
const WAT: &str = r#"(module
  (import "host" "double" (func $double (param i32) (result i32)))
  (memory 1)
  (func (export "run") (param i32) (result i32)
    local.get 0
    call $double
    i32.const 1
    i32.add)
  (func (export "spin")
    (loop $l (br $l)))
  (func (export "grow") (param i32) (result i32)
    local.get 0
    memory.grow))"#;

/// Resource limits for one evaluation.
struct SandboxConfig {
    /// Maximum number of units of "fuel" (roughly, Wasm instructions).
    fuel: u64,
    /// Maximum size of the linear memory, in bytes.
    max_memory_bytes: usize,
}

/// An engine that meters fuel, plus a cache of compiled modules.
struct Sandbox {
    engine: Engine,
    config: SandboxConfig,
    /// Compiled modules, keyed by the hash of their source.
    cache: HashMap<String, Module>,
}

impl Sandbox {
    fn new(config: SandboxConfig) -> Result<Self> {
        let mut engine_config = Config::new();
        engine_config.consume_fuel(true);
        Ok(Self {
            engine: Engine::new(&engine_config)?,
            config,
            cache: HashMap::new(),
        })
    }

    /// Compiles WAT (or reuses the cached module). A `Module` is only valid
    /// for the `Engine` (and compatible `Config`) that compiled it.
    fn module(&mut self, wat: &str) -> Result<Module> {
        let key = blake3::hash(wat.as_bytes()).to_hex().to_string();
        if let Some(module) = self.cache.get(&key) {
            return Ok(module.clone());
        }
        let module = Module::new(&self.engine, wat::parse_str(wat)?)?;
        self.cache.insert(key, module.clone());
        Ok(module)
    }

    /// A fresh store: each evaluation gets its own fuel and memory limit.
    fn store(&self) -> Result<Store<StoreLimits>> {
        let limits = StoreLimitsBuilder::new()
            .memory_size(self.config.max_memory_bytes)
            .build();
        let mut store = Store::new(&self.engine, limits);
        store.limiter(|limits| limits);
        store.set_fuel(self.config.fuel)?;
        Ok(store)
    }

    /// Calls an exported `(i32) -> i32` function.
    fn call(&mut self, wat: &str, export: &str, arg: i32) -> Result<i32> {
        let module = self.module(wat)?;
        let mut linker = Linker::new(&self.engine);
        // The only capability granted to the guest:
        linker.func_wrap("host", "double", |x: i32| x.wrapping_mul(2))?;
        let mut store = self.store()?;
        let instance = linker.instantiate(&mut store, &module)?;
        let func = instance.get_typed_func::<i32, i32>(&mut store, export)?;
        Ok(func.call(&mut store, arg)?)
    }

    /// Calls an exported `() -> ()` function.
    fn call_unit(&mut self, wat: &str, export: &str) -> Result<()> {
        let module = self.module(wat)?;
        let linker = {
            let mut linker = Linker::new(&self.engine);
            linker.func_wrap("host", "double", |x: i32| x.wrapping_mul(2))?;
            linker
        };
        let mut store = self.store()?;
        let instance = linker.instantiate(&mut store, &module)?;
        let func = instance.get_typed_func::<(), ()>(&mut store, export)?;
        Ok(func.call(&mut store, ())?)
    }
}

fn main() -> Result<()> {
    let mut sandbox = Sandbox::new(SandboxConfig {
        fuel: 10_000,
        max_memory_bytes: 2 * 0x0001_0000, // Two Wasm pages.
    })?;

    // A normal call.
    assert_eq!(sandbox.call(WAT, "run", 20)?, 41);

    // An infinite loop runs out of fuel instead of hanging the host.
    let err = sandbox.call_unit(WAT, "spin").unwrap_err();
    assert_eq!(err.downcast_ref::<Trap>(), Some(&Trap::OutOfFuel));

    // Growing past the limit is refused: `memory.grow` returns -1.
    assert_eq!(sandbox.call(WAT, "grow", 1)?, 1); // 1 -> 2 pages: allowed.
    assert_eq!(sandbox.call(WAT, "grow", 5)?, -1); // Too big: denied.
    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }

    #[test]
    fn modules_are_cached_by_content() -> Result<()> {
        let mut sandbox = Sandbox::new(SandboxConfig {
            fuel: 10_000,
            max_memory_bytes: 0x0001_0000,
        })?;
        sandbox.module(WAT)?;
        sandbox.module(WAT)?;
        assert_eq!(sandbox.cache.len(), 1);
        Ok(())
    }
}
