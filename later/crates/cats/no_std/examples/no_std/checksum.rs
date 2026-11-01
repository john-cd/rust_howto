// TODO Reviewed ??-??-2026.

//! A host binary that calls the library's core-only checksum implementation.
//!
//! `no_std` code uses only the `core` library: no operating system and no heap.
//! It suits firmware, kernels and embedded boards. A checksum is a small value
//! computed from data to detect accidental corruption, but it does not protect
//! against deliberate tampering.

// ANCHOR: example
use no_std::checksum;

/// Calls the same checksum function with wrapping, empty, and ordinary inputs.
fn main() {
    assert_eq!(checksum(&[250, 10]), 4);
    assert_eq!(checksum(&[]), 0);
    println!("Checksum of Rust: {}", checksum(b"Rust"));
}
// ANCHOR_END: example

/// Runs the host demonstration; the library itself does not need a host
/// runtime.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the no std1 example through its entry point.
    #[test]
    fn test_no_std1_example_entry_point() {
        main();
    }

    #[test]
    fn checksum_wraps_modulo_256() {
        assert_eq!(checksum(&[255, 1]), 0);
        assert_eq!(checksum(&[1, 2, 3]), 6);
        assert_eq!(checksum(b"Rust"), 174);
    }
}
