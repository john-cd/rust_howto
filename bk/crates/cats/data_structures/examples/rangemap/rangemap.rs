// TODO Reviewed ??-??-2026
// ANCHOR: example
//! A `RangeMap`, which maps ranges of keys to values.

use rangemap::RangeMap;

/// Maps two ranges to values and looks up keys.
fn main() {
    let mut rm = RangeMap::new();

    rm.insert(0..50, "first half");
    rm.insert(50..100, "second half");

    assert_eq!(rm.get(&25), Some(&"first half"));
    assert_eq!(rm.get(&75), Some(&"second half"));
    assert_eq!(rm.get(&100), None);
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
