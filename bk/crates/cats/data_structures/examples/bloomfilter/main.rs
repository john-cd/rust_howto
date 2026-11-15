//! Runs the Bloom filter example.
//!
//! TODO Reviewed ??-??-2026.

/// A Bloom filter, a compact set that can give false positives but no false
mod bloomfilter;
/// Runs the example.
/// Runs the bloomfilter example.
fn main() {
    bloomfilter::run();
}

/// Verifies that the Bloom filter example.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the Bloom filter example's default dispatcher.
    #[test]
    fn bloomfilter_dispatcher_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
