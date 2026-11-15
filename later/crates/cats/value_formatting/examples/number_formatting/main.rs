//! Runs the number formatting example.
//!
//! TODO Reviewed ??-??-2026.

/// Format a byte count as a human-readable size.
mod humansize_example;
/// Formats an integer with the `itoa` crate.
mod itoa;
/// # Ryū Example This example demonstrates the usage of the `ryu` crate, a pure Rust implementation
/// of the Ryū algorithm for fast floating-point to decimal string conversion.
mod ryu;
/// Demonstrates standard Rust format syntax and locale-aware integer grouping.
mod standard_and_localized;
/// Runs the example.
/// Runs the number formatting example.
fn main() {
    humansize_example::run();
    itoa::run();
    ryu::run();
    standard_and_localized::run();
}
