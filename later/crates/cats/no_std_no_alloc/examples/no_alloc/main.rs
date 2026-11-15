//! Runs the no alloc example.
//!
//! TODO Reviewed ??-??-2026.

/// A no-allocation example using fixed-capacity collections.
mod no_alloc;
/// Runs the no alloc example.

/// # Errors
/// Propagates fixed-capacity string formatting failures.
fn main() -> core::fmt::Result {
    no_alloc::run()
}

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
