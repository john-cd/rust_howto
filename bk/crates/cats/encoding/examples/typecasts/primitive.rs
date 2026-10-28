#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates explicit primitive casts, checked numeric conversions, and
//! parsing.

use anyhow::Result;

/// Converts a signed integer to `i32`, returning an error when it is out of
/// range.
fn narrow_to_i32(value: i64) -> Result<i32, std::num::TryFromIntError> {
    i32::try_from(value)
}

/// Runs examples of primitive conversions.
fn main() -> Result<()> {
    let count = narrow_to_i32(42_i64)?;
    let port = "443".parse::<u16>()?;
    let wide_port: u32 = port.into();

    // This cast intentionally discards the high byte: 0x1234 becomes 0x34.
    let low_byte = 0x1234_u16 as u8;
    assert_eq!(low_byte, 0x34);

    println!("count={count}, port={wide_port}, low byte={low_byte:#04x}");
    Ok(())
}

// ANCHOR_END: example

/// Runs the conversion examples.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that narrowing a value outside the destination range fails.
    #[test]
    fn narrowing_rejects_out_of_range_values() {
        let too_large = i64::from(i32::MAX) + 1;
        assert!(narrow_to_i32(too_large).is_err());
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
