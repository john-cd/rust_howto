// TODO Reviewed ??-??-2026.

//! A host binary that calls the library's core-only checksum implementation.

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
}
