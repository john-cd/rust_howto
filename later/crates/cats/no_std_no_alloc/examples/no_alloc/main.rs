//! Runs the no alloc example.
//!
//! TODO Reviewed ??-??-2026.

mod no_alloc;

/// Runs the no alloc example.
///
/// # Errors
/// Propagates fixed-capacity string formatting failures.
fn main() -> core::fmt::Result {
    no_alloc::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
