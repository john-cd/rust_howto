//! Runs the vector example.
//!
//! TODO Reviewed ??-??-2026.

/// This example demonstrates vector creation, manipulation, iteration, and slices in Rust.
mod vectors;

/// Entry point for the vector example.
fn main() {
    vectors::run();
}

/// Verifies that the vector example.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the vector dispatcher runs.
    #[test]
    fn vector_dispatcher_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
