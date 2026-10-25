//! Runs the number formatting example.
//!
//! TODO Reviewed ??-??-2026.

mod humansize_example;
mod itoa;
mod ryu;
mod standard_and_localized;

/// Runs the number formatting example.
fn main() {
    humansize_example::run();
    itoa::run();
    ryu::run();
    standard_and_localized::run();
}
