//! Runs the smart pointer example.
//!
//! TODO Reviewed ??-??-2026.

/// Implementing `Deref` for a smart pointer.
mod deref;
/// Runs the example.
/// Runs the smart pointers example.

fn main() {
    deref::run();
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
