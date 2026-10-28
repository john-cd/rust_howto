// TODO Reviewed ??-??-2026.

//! A no-allocation example using fixed-capacity collections.

// ANCHOR: example
use core::fmt;
use core::fmt::Write as _;

use heapless::String;
use heapless::Vec;

/// Demonstrates fixed-capacity storage and formatting without heap allocation.
///
/// # Errors
/// Returns a formatting error if the fixed-capacity message is too small.
fn main() -> fmt::Result {
    let mut samples: Vec<u8, 8> = Vec::new();
    for sample in 0..8 {
        assert!(samples.push(sample).is_ok());
    }
    // A full vector returns ownership of the rejected item and stays unchanged.
    if let Err(rejected) = samples.push(8) {
        assert_eq!(rejected, 8);
        assert_eq!(samples.as_slice(), &[0, 1, 2, 3, 4, 5, 6, 7]);
    }

    let mut message: String<32> = String::new();
    write!(&mut message, "sample count: {}", samples.len())?;
    assert_eq!(message.as_str(), "sample count: 8");
    Ok(())
}
// ANCHOR_END: example

/// Runs the no-allocation demonstration in the host binary.
///
/// # Errors
/// Propagates a formatting error if the message does not fit its inline buffer.
pub fn run() -> fmt::Result {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the example rejects excess items and formats into fixed
    /// storage.
    #[test]
    fn fixed_capacity_storage_rejects_overflow_and_formats() {
        main().expect("the demonstrated message fits in 32 bytes");
    }
}
