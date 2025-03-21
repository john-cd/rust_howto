// ANCHOR: example
use anyhow::Context;
use anyhow::Result;
use wasmtime::*;

// Simple example written in WAT (WebAssembly Text Format).
// Imports a host function; exports a function as well.
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

fn main() -> Result<()> {
    // Create a WASM engine.
    let engine = Engine::default();

    // Create a module from WAT:
    let module = Module::new(&engine, WAT)?;

    // Or load a module from a WASM file.
    // You may store the WAT into a file and compile it using `wat2wasm`.
    // Install that tool with e.g. `sudo apt-get install wabt`.
    // let module = Module::from_file(&engine, "hello.wasm").context("Failed to
    // create WASM module")?;

    // Host functionality can be arbitrary Rust functions and is provided
    // to guests through a `Linker`.
    let mut linker = Linker::new(&engine);

    // Define the host function "print_char"
    linker.func_wrap("host", "print_char", |x: i32| {
        print!("{}", char::from_u32(x as u32).unwrap_or('?')); // Handle invalid char codes
    })?;

    // All WASM objects operate within the context of a "store".
    // `Store` has a type parameter to store host-specific data.
    let mut store = Store::new(&engine, ());

    // Create an instance, linking the imports
    let instance = linker
        .instantiate(&mut store, &module)
        .context("failed to instantiate module")?;

    // Without linker, you may also use:
    // let instance = Instance::new(&mut store, &module, &[])
    //    .context("Failed to create WASM instance")?;

    // Get the exported function "greet".
    let greet = instance
        .get_export(&mut store, "greet")
        .and_then(|ext: Extern| {
            ext.into_func() // Returns the underlying Func, if this external is a function.
        })
        .context("Failed to find `greet` function export")?;

    // Or:
    // let greet = instance.get_typed_func::<(), ()>(&mut store, "greet")?;

    // Call the "greet" function.
    greet.call(&mut store, &[], &mut []) // store, params, result
        .context("failed to call `greet` function")?;

    // Example with arguments
    let add = instance
        .get_export(&mut store, "add")
        .and_then(|ext: Extern| ext.into_func())
        .context("failed to find `add` function")?;

    let mut results = [Val::I32(0)];
    add.call(&mut store, &[Val::I32(10), Val::I32(20)], &mut results)
        .context("failed to call `add` function")?;

    println!("\nResult of add function: {}", results[0].unwrap_i32());

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [review](https://github.com/john-cd/rust_howto/issues/855)
// review https://docs.wasmtime.dev/introduction.html
// https://docs.rs/wasmtime/latest/wasmtime/
