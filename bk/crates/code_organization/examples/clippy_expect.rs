//! Explains a scoped Clippy expectation using a four-bit invariant.
//!
//! TODO Reviewed ??-??-2026.

#![warn(clippy::expect_used)]

// ANCHOR: example
/// Converts the low four bits of a byte to a hexadecimal digit.
#[expect(clippy::expect_used, reason = "The mask restricts the value to 0–15.")]
fn low_hex_digit(byte: u8) -> char {
    char::from_digit(u32::from(byte & 0x0f), 16)
        .expect("a four-bit value fits in one hexadecimal digit")
}

/// Prints the low hexadecimal digit.
fn main() {
    println!("{}", low_hex_digit(0xab));
}
// ANCHOR_END: example

/// The upper bits do not affect the hexadecimal digit.
#[test]
fn hexadecimal_digit_uses_only_low_bits() {
    main();
    assert_eq!(low_hex_digit(0xab), 'b');
    assert_eq!(low_hex_digit(0xff), 'f');
    assert_eq!(low_hex_digit(0xf0), '0');
}
