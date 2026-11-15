//! Runs the `Pin` example.
//!
//! TODO Reviewed ??-??-2026.

/// Pinning values with `Pin` and `PhantomPinned`.
mod pin;
/// Runs the example.
/// Runs the pin example.

fn main() {
    pin::run();
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
