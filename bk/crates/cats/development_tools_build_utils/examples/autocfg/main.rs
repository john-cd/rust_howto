//! Autocfg examples.
//!
//! TODO Reviewed ??-??-2026.

/// `autocfg` probes compiler support for use in Cargo build scripts.
mod autocfg;
/// Runs the example.
/// Runs the autocfg example.
fn main() {
    autocfg::run();
}

/// panic fails the test.
#[cfg(test)]
mod tests {
    use super::*;

    /// panic fails the test.
    #[test]
    fn test_main_runs() {
        main();
    }
}
