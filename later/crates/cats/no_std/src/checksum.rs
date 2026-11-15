// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Wrapping byte arithmetic without allocation or operating-system services.

/// Returns the sum of all bytes modulo 256; an empty slice has checksum zero.
///
/// # Arguments
/// * `bytes` - Input bytes to decode or write.
pub fn checksum(bytes: &[u8]) -> u8 {
    bytes.iter().fold(0_u8, |sum, byte| sum.wrapping_add(*byte))
}
// ANCHOR_END: example

/// Calculates the example checksum for a short byte string.
fn run() -> u8 {
    checksum(b"Rust")
}

/// services.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_wraps_and_handles_empty_input() {
        assert_eq!(checksum(&[]), 0);
        assert_eq!(checksum(&[255, 1]), 0);
    }
}
