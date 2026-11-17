//! One extension crate linked into the registry example.

use plugin_registry::Command;

/// Handles the greet behavior for this component.
fn greet() -> &'static str {
    "hello"
}

inventory::submit! { Command { name: "greet", run: greet } }

/// Explicitly touch the extension crate so its registration is linked.
pub fn linked() {}
