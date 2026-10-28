// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Formats an integer with the `itoa` crate.

use itoa::Buffer;

/// Formats an integer into a reusable stack buffer.
fn main() {
    let mut buffer = Buffer::new();
    let formatted = buffer.format(1_234_567);
    assert_eq!(formatted, "1234567");
    println!("{formatted}");
}

// ANCHOR_END: example

#[allow(clippy::main_recursion)]
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that `itoa` renders an integer without grouping separators.
    #[test]
    fn integer_output_is_stable() {
        let mut buffer = Buffer::new();
        assert_eq!(buffer.format(1_234_567), "1234567");
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
