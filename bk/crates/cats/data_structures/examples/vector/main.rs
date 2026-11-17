//! Runs the vector example.
//!
//! TODO Reviewed ??-??-2026.

/// This example demonstrates vector creation, manipulation, iteration, and slices in Rust.
mod vectors;

/// Entry point for the vector example.
fn main() {
    vectors::run();
}

/// Provides vector capacity behavior for the vector component.
#[cfg(test)]
mod vector_capacity;
