//! Runs the money examples.
//!
//! TODO Reviewed ??-??-2026.

/// Represent, add, and format money with an explicit currency.
mod rusty_money;
/// Runs the example.
/// Runs the money examples.
fn main() {
    rusty_money::run();
}

/// tests its own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// tests its own results).
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
