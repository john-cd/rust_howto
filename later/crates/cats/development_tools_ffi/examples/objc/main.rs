//! Runs the Objective-C interoperability example.
//!
//! TODO Reviewed ??-??-2026.

/// Convert a Rust string with the typed Objective-C Foundation bindings.
mod objc2;
/// Runs the example.
/// Runs the objc example.
fn main() {
    objc2::run();
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
