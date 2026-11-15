//! Runs the reference counting example.
//!
//! TODO Reviewed ??-??-2026.

/// `Rc` is a smart pointer that allows multiple owners of the same data.
mod rc;
/// Runs the example.
/// Runs the reference counting example.
fn main() {
    rc::run();
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
