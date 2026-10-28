//! Runs the test runner example.
//!
//! TODO Reviewed ??-??-2026.

mod tests1;

/// Runs the test runners example.
fn main() {
    tests1::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
