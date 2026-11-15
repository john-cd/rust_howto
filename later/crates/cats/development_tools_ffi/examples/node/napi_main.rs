//! Runs the N-API example.
//!
//! TODO Reviewed ??-??-2026.

/// Calling Rust from Node.js with `napi`.
mod napi;

/// Runs the N-API example's host-independent path.
fn main() {
    napi::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn test_main_entry_point() {
        main();
        assert_eq!(napi::sum(20, 22), 42);
    }
}
