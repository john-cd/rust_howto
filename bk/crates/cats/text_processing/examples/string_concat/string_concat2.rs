// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Concatenates constant string slices at compile time with `constcat`.
//!
//! Constants are computed at compile time, so a combined message costs nothing
//! at run time. The standard `concat!` macro handles literals; the `constcat`
//! crate also combines constants.

use constcat::concat;

/// The date portion of the timestamp.
const DATE: &str = "2024-01-15";
/// The timestamp separator.
const SEPARATOR: &str = "T";
/// The time portion of the timestamp.
const TIME: &str = "12:00:09Z";
/// The timestamp assembled during compilation.
const DATETIME: &str = concat!(DATE, SEPARATOR, TIME);

/// Prints the compile-time concatenated timestamp.
fn main() {
    println!("{DATETIME}");
}

// ANCHOR_END: example

/// Runs the compile-time concatenation example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::DATETIME;
    use super::main;

    /// Verifies constant slices were concatenated into the expected timestamp.
    /// Runs the constcat example through its entry point.
    #[test]
    fn concatenates_constant_slices() {
        assert_eq!(DATETIME, "2024-01-15T12:00:09Z");
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
