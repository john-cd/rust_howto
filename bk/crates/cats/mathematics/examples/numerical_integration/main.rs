//! Runs the numerical integration example.
//!
//! TODO Reviewed ??-??-2026.

/// Demonstrates dependency-free numerical integration.
mod quadrature;

/// Prints the trapezoidal and Simpson estimates.
fn main() {
    quadrature::run();
}
