#![allow(dead_code)]
// ANCHOR: example
//! Simple `wasmi` example for virtualization.
//!
//! This loads a tiny WebAssembly module from a raw byte array, instantiates it
//! with `wasmi`, and calls the exported `run` function.
use wasmi::ImportsBuilder;
use wasmi::Module;
use wasmi::ModuleInstance;
use wasmi::NopExternals;

fn main() {
    let wasm_bytes: &[u8] = &[
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x04, 0x01, 0x60,
        0x00, 0x01, 0x7f, 0x03, 0x02, 0x01, 0x00, 0x07, 0x07, 0x01, 0x03, 0x72,
        0x75, 0x6e, 0x00, 0x00, 0x0a, 0x09, 0x01, 0x07, 0x00, 0x41, 0x2a, 0x0b,
    ];

    let module =
        Module::from_buffer(wasm_bytes).expect("failed to load wasm module");
    let instance = ModuleInstance::new(&module, &ImportsBuilder::default())
        .expect("failed to instantiate wasm module")
        .assert_no_start();

    let result = instance
        .invoke_export("run", &[], &mut NopExternals)
        .expect("failed to invoke exported function");

    let value = result.expect_i32();
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
