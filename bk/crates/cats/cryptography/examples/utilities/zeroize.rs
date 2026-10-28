// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates the use of the `zeroize` crate to securely clear sensitive data
//! from memory.

// This example demonstrates how to securely clear sensitive data from memory
// using the `zeroize` crate.
use zeroize::Zeroize as _;

/// Demonstrates the use of the `zeroize` crate to securely clear sensitive data
/// from memory.
fn main() {
    // Example bytes only; never print real keys or passwords.
    let mut sensitive_data = [1_u8, 2, 3, 4, 5, 6, 7, 8];
    sensitive_data.zeroize();
    assert_eq!(sensitive_data, [0; 8]);
}

// ANCHOR_END: example

/// Runs the zeroization example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the zeroize example through its entry point.
    #[test]
    fn test_zeroize_example_entry_point() {
        main();
    }
}
