//! Runs the `accesskit` accessibility example.
//!
//! Reviewed 10-27-2026.
//!
//! TODO Reviewed ??-??-2026.

/// Describes a small interface to screen readers with AccessKit.
mod accesskit;
/// Runs the example.
/// Runs the screen readers example.
fn main() {
    accesskit::run();
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
