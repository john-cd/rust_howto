//! Wrapping byte arithmetic without allocation or operating-system services.

// ANCHOR: example
/// Returns the sum of all bytes modulo 256; an empty slice has checksum zero.
pub fn checksum(bytes: &[u8]) -> u8 {
    bytes.iter().fold(0_u8, |sum, byte| sum.wrapping_add(*byte))
}
// ANCHOR_END: example
