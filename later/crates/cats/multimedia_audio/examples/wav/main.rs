//! Runs the in-memory WAV example.
//!
//! TODO Reviewed ??-??-2026.

/// Writes and reads a WAV file through an in-memory cursor.
mod wav;
/// Runs the example.
/// Runs the in-memory WAV example.

fn main() {
    wav::run().expect("writing and reading a WAV buffer should succeed");
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
