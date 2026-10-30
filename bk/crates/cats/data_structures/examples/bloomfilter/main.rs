//! Runs the Bloom filter example.
//!
//! TODO Reviewed ??-??-2026.

mod bloomfilter;

/// Runs the bloomfilter example.
fn main() {
    bloomfilter::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the Bloom-filter entry point runs its checked example.
    #[test]
    fn bloomfilter_dispatcher_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
