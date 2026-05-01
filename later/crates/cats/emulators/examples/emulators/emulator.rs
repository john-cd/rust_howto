#![allow(dead_code)]
// ANCHOR: example
use polkavm::{Config, Engine};

fn main() {
    // 1. Setup the VM engine
    let config = Config::default();
    let engine = Engine::new(&config).expect("Failed to create engine");

    // PolkaVM is a fast RISC-V based virtual machine.
    // This example demonstrates how to initialize the execution engine.
    // In a real application, you would load a ProgramBlob, create a Module,
    // and then instantiate it to run code.

    println!("PolkaVM engine initialized successfully.");
    let _ = engine;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
