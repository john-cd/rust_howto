#![allow(dead_code)]
// ANCHOR: example
// This example demonstrates how to securely clear sensitive data from memory
// using the `zeroize` crate.
use zeroize::Zeroize;

/// Demonstrates the use of the `zeroize` crate to securely clear sensitive data
/// from memory.
fn main() {
    // Create a vector of sensitive data, e.g., a password or key.
    let mut sensitive_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    // Print the sensitive data before zeroizing.
    println!("Sensitive data before zeroizing: {sensitive_data:?}");

    // Securely zeroize the sensitive data:
    sensitive_data.zeroize();

    println!("Sensitive data after zeroizing: {sensitive_data:?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main();
    }
}
