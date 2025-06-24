#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `wasmer` crate to run a WebAssembly
//! module.
//!
//! The WebAssembly module is defined in the WebAssembly Text (WAT) format.
//! It exports a function named `add` that takes two `i32` parameters and
//! returns their sum.
use wasmer::Instance;
use wasmer::Module;
use wasmer::Store;
use wasmer::imports;

fn main() -> anyhow::Result<()> {
    // Load a WebAssembly module from the WebAssembly Text (WAT) format.
    // This example will simply add two numbers:
    let module_wat = r#"
    (module
      (type $add_t (func (param i32 i32) (result i32)))
      (func $add_f (type $add_t) (param $lhs i32) (param $rhs i32) (result i32)
        local.get $lhs
        local.get $rhs
        i32.add)
      (export "add" (func $add_f)))"#;
    // You may also use `wat::parse_str(...) or parse_file(...)`, e.g.
    // let wasm_bytes = wat::parse_file("./foo.wat")?;

    // Create a store.
    // The store represents all global state that can be manipulated by
    // WebAssembly programs. It consists of the runtime representation of
    // all instances of functions, tables, memories, and globals
    // that have been allocated during the lifetime of the abstract machine.
    let mut store = Store::default();

    // Compile the module.
    // The Store also holds the engine that is used to compile the WASM bytes
    // into a valid module artifact.
    let module = Module::new(&store, module_wat)?;

    // Create an import object.
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};

    // Instantiate the module:
    let instance = Instance::new(&mut store, &module, &import_object)?;

    // Get the `add` function from the exports.
    let add = instance.exports.get_function("add")?;

    // Call the function.
    let result = add.call(&mut store, &[1.into(), 2.into()])?;
    println!("1 + 2 = {}", result[0].i32().unwrap());

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
