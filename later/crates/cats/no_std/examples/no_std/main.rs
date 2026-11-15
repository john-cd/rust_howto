//! Runs the no std example.
//!
//! TODO Reviewed ??-??-2026.

/// A host binary that calls the library's core-only checksum implementation.
mod checksum;
/// Runs the checksum demonstration on the host.
fn main() {
    checksum::run();
}

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
