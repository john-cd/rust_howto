//! Runs the Flutter interoperability example.
//!
//! TODO Reviewed ??-??-2026.

/// Exposes the Flutter Rust Bridge example API and runner.
pub mod flutter_rust_bridge;

/// Runs the flutter example.
fn main() {
    flutter_rust_bridge::run();
}
