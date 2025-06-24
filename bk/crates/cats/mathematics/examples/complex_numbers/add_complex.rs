#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates the manipluation of complex numbers.

fn main() {
    // Create two complex numbers.
    let complex_num1 = num::complex::Complex::new(10.0, 20.0);
    let complex_num2 = num::complex::Complex::new(3.1, -4.2);

    // Add the two complex numbers.
    let sum = complex_num1 + complex_num2;

    println!("Sum: {}", sum);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
