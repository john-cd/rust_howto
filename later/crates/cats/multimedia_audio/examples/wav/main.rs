//! Runs the in-memory WAV example.

mod wav;

/// Runs the in-memory WAV example.
fn main() {
    wav::run().expect("writing and reading a WAV buffer should succeed");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
