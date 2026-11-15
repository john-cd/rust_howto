//! Runs examples of explicit integer overflow behavior.
//!
//! TODO Reviewed ??-??-2026.

/// Demonstrates explicit primitive integer overflow operations.
mod overflow_methods;

/// Demonstrates wrapping arithmetic with `std::num::Wrapping`.
mod wrapping_type;

/// Runs the primitive methods and `Wrapping` examples.
fn main() {
    overflow_methods::run();
    wrapping_type::run();
}
