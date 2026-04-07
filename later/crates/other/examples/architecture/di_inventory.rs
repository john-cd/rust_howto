#![allow(dead_code)]
// ANCHOR: example
//! The [`inventory`][c~inventory~docs]↗ crate provides a mechanism to
//! register values globally at compile time, which is useful for building
//! plugin systems, service registries, and lightweight dependency injection.
//!
//! Unlike runtime DI frameworks, `inventory` collects registered items at
//! link time, so all plugins are known at compile time.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! inventory = "0.3"
//! ```

/// Describes a plugin's metadata and behavior.
///
/// Each plugin registers a `PluginRegistration` value via
/// `inventory::submit!`. The `execute` field is a function pointer, which
/// avoids the `Sized` limitation of trait objects with `inventory`.
pub struct PluginRegistration {
    pub name: &'static str,
    pub execute: fn(),
}

// Declare `PluginRegistration` as an `inventory`-collectable type.
inventory::collect!(PluginRegistration);

fn hello_execute() {
    println!("Hello from the HelloPlugin!");
}

fn goodbye_execute() {
    println!("Goodbye from the GoodbyePlugin!");
}

// Register each plugin at link time.
inventory::submit!(PluginRegistration {
    name: "hello",
    execute: hello_execute,
});

inventory::submit!(PluginRegistration {
    name: "goodbye",
    execute: goodbye_execute,
});

fn main() {
    println!("Registered plugins:");
    // Iterate over all registered plugins at runtime.
    for plugin in inventory::iter::<PluginRegistration> {
        println!("  - {}", plugin.name);
        (plugin.execute)();
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
