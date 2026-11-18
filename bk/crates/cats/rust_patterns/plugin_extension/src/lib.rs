//! One extension crate linked into the registry example.

use plugin_registry::Command;

/// Returns the extension greeting when its inventory-registered command is invoked.
fn greet() -> &'static str {
    "hello"
}

inventory::submit! { Command { name: "greet", run: greet } }

/// Explicitly touch the extension crate so its registration is linked.
pub fn linked() {}
