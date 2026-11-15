//! Runs examples of explicit integer overflow behavior.
//!
//! TODO Reviewed ??-??-2026.

/// Demonstrates explicit primitive integer overflow operations.
mod overflow_methods;
/// Demonstrates wrapping arithmetic with `std::num::Wrapping`.
mod wrapping_type;
/// Runs the example.
/// Runs the primitive methods and `Wrapping` examples.
fn main() {
    overflow_methods::run();
    wrapping_type::run();
}

/// Runs both overflow examples and their assertions.
#[cfg(test)]
mod tests {
    /// Runs both overflow examples and their assertions.
    #[test]
    fn test_examples_run() {
        super::main();
    }
}
