//! Runnable entry point for the smoltcp example.
/// Provides ipv4 subnet behavior for the smoltcp component.
mod ipv4_subnet;

/// Runs the example demonstrated by this module.
fn main() {
    ipv4_subnet::run();
}
