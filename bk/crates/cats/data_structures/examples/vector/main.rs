//! Runs the vector example.
//!
//! TODO Reviewed ??-??-2026.

mod vectors;

/// Entry point for the vector example.
fn main() {
    vectors::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the vector entry point runs its checked example.
    #[test]
    fn vector_dispatcher_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
