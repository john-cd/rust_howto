#![allow(unused)]
// ANCHOR: example
use proc_macros::DebugPrint;

// Define the custom trait that the macro will implement.
pub trait DebugPrint {
    fn debug_print(&self);
}

// Apply our custom derive macro.
// This will automatically `impl DebugPrint for` the struct.
// We also derive the standard `Debug` trait.
#[derive(DebugPrint, Debug)]
struct User {
    id: u64,
    name: String,
    active: bool,
}

fn main() {
    let u = User {
        id: 1,
        name: "Alice".to_string(),
        active: true,
    };
    u.debug_print();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
