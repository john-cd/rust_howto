//! Placeholder for a code coverage example.
//!
//! TODO Reviewed ??-??-2026.

// Deferred (#1748): no coverage example yet; see the code coverage chapter.

/// Does nothing yet.
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    /// The example is deferred (#1748); `main` has nothing to run yet.
    #[test]
    fn test_main_does_nothing() {
        // output-only: the placeholder prints nothing and cannot fail.
        main();
    }
}
