//! Runs the Flutter interoperability example.
//!
//! TODO Reviewed ??-??-2026.

/// Exposes the Flutter Rust Bridge example API and runner.
pub mod flutter_rust_bridge;

/// Runs the flutter example.
fn main() {
    flutter_rust_bridge::run();
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
