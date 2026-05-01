#![allow(dead_code)]
// ANCHOR: example
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

#[test]
fn test() {
    main();
}
