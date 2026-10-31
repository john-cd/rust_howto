// ANCHOR: example
//! Converts an `f32` value to the compact IEEE 754 binary16 `f16` type.
//!
//! `f16` uses 16 bits instead of 32 for a floating-point number. It has less
//! range and precision but halves memory and bandwidth, which is why machine
//! learning models and graphics often store data in it.

use half::f16;

/// Shows that converting a value to half precision can lose information.
fn main() {
    let original = 1.0001_f32;
    let compact = f16::from_f32(original);
    let restored = compact.to_f32();

    println!("{original} stored as f16 and converted back is {restored}");
    assert_eq!(restored, 1.0);
    assert_ne!(restored, original);
}

// ANCHOR_END: example

/// Runs the half-precision example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that binary16 conversion rounds a value that is not
    /// representable.
    #[test]
    fn rounds_when_converting_to_half_precision() {
        main();
    }
}
