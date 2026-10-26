#![allow(dead_code)]
// TODO Reviewed ??-??-2026

//! A no-allocation example using fixed-capacity collections.

use core::fmt::Write as _;

use heapless::String;
use heapless::Vec;

// ANCHOR: example
/// Demonstrates fixed-capacity storage and formatting without heap allocation.
fn main() {
    let mut samples: Vec<u8, 8> = Vec::new();
    for sample in 0..8 {
        assert!(samples.push(sample).is_ok());
    }
    assert_eq!(samples.push(8), Err(8));

    let mut message: String<32> = String::new();
    write!(&mut message, "sample count: {}", samples.len())
        .expect("the message fits in the fixed-capacity string");
    assert_eq!(message.as_str(), "sample count: 8");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the example rejects excess items and formats into fixed
    /// storage.
    #[test]
    fn fixed_capacity_storage_rejects_overflow_and_formats() {
        main();
    }
}
