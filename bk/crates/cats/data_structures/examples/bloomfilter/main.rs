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

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
