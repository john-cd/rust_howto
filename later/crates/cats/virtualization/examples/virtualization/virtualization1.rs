#![allow(dead_code)]
// ANCHOR: example
//! Simple `wasmi` example for virtualization.
//!
//! This loads a tiny WebAssembly module from a raw byte array, instantiates it
//! with `wasmi`, and calls the exported `run` function.
use wasmi::Engine;
use wasmi::Linker;
use wasmi::Module;
use wasmi::Store;

fn main() {
    let wasm_bytes: &[u8] = &[
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x04, 0x01, 0x60,
        0x00, 0x01, 0x7f, 0x03, 0x02, 0x01, 0x00, 0x07, 0x07, 0x01, 0x03, 0x72,
        0x75, 0x6e, 0x00, 0x00, 0x0a, 0x09, 0x01, 0x07, 0x00, 0x41, 0x2a, 0x0b,
    ];

    let engine = Engine::default();
    let module =
        Module::new(&engine, wasm_bytes).expect("failed to load wasm module");
    let mut store = Store::new(&engine, ());
    let linker = Linker::new(&engine);
    let instance = linker
        .instantiate_and_start(&mut store, &module)
        .expect("failed to instantiate/start wasm module");

    let run = instance
        .get_typed_func::<(), i32>(&store, "run")
        .expect("failed to resolve exported function");
    let value = run
        .call(&mut store, ())
        .expect("failed to invoke exported function");
    println!("wasmi result: {}", value);
}
// ANCHOR_END: example
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        main();
    }
}
