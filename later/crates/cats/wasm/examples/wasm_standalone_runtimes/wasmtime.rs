#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to use the `wasmtime` crate to run WebAssembly
//! modules.
//!
//! It showcases loading a module from WAT (WebAssembly Text Format), defining
//! host functions, instantiating the module, and calling exported functions.

use anyhow::Result;
use wasmtime::error::Context as _;
use wasmtime::*;

/// A WebAssembly module in text format, which imports and exports a function.
static WAT: &str = r#"(module
  (import "host" "print_char" (func $print_char (param i32)))

  (func $greet (export "greet")
    (local $i i32)
    (local.set $i (i32.const 0))
    loop $loop
      local.get $i
      i32.const 12
      i32.lt_s
      if
        local.get $i
        i32.const 72
        i32.add
        call $print_char

        local.get $i
        i32.const 1
        i32.add
        local.set $i
        br $loop
      end
    end
  )

    (func $add (export "add") (param i32 i32) (result i32)
        local.get 0
        local.get 1
        i32.add
    )
)"#;

/// Creates a Wasmtime Engine, parses WAT, links host functions, and returns an Instance.
fn create_and_instantiate_wasm(wat_str: &str) -> Result<(Engine, Store<()>, Instance)> {
    let engine = Engine::default();
    let module = Module::new(&engine, wat_str)?;
    let mut linker = Linker::new(&engine);

    linker.func_wrap("host", "print_char", |x: i32| {
        print!("{}", char::from_u32(x as u32).unwrap_or('?'));
    })?;

    let mut store = Store::new(&engine, ());
    let instance = linker
        .instantiate(&mut store, &module)
        .context("failed to instantiate module")?;

    Ok((engine, store, instance))
}

/// Runs the WebAssembly module with `wasmtime`, linking a host function.
fn main() -> Result<()> {
    let (_engine, mut store, instance) = create_and_instantiate_wasm(WAT)?;

    let greet = instance
        .get_export(&mut store, "greet")
        .and_then(|ext: Extern| ext.into_func())
        .context("Failed to find `greet` function export")?;

    greet
        .call(&mut store, &[], &mut [])
        .context("failed to call `greet` function")?;

    let add = instance
        .get_export(&mut store, "add")
        .and_then(|ext: Extern| ext.into_func())
        .context("failed to find `add` function")?;

    let mut results = [Val::I32(0)];
    add.call(&mut store, &[Val::I32(10), Val::I32(20)], &mut results)
        .context("failed to call `add` function")?;

    println!("\nResult of add function: {}", results[0].unwrap_i32());
    assert_eq!(results[0].unwrap_i32(), 30);

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the wasmtime example through its entry point.
    #[test]
    fn test_wasmtime_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    #[test]
    fn test_create_and_instantiate_wasm() -> anyhow::Result<()> {
        let res = create_and_instantiate_wasm(WAT);
        assert!(res.is_ok());

        let invalid_wat = "(module (invalid syntax))";
        assert!(create_and_instantiate_wasm(invalid_wat).is_err());
        Ok(())
    }
}
