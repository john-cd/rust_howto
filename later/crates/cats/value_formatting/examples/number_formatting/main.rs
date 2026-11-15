//! Runs the number formatting example.
//!
//! TODO Reviewed ??-??-2026.

/// Format a byte count as a human-readable size.
mod humansize_example;

/// Formats an integer with the `itoa` crate.
mod itoa;

/// Converts floating-point values to decimal strings with the `ryu` crate.
mod ryu;

/// Demonstrates standard Rust format syntax and locale-aware integer grouping.
mod standard_and_localized;

/// Runs the number formatting example.
fn main() {
    humansize_example::run();
    itoa::run();
    ryu::run();
    standard_and_localized::run();
}
