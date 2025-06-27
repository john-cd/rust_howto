#![allow(dead_code)]
// ANCHOR: example
//! The `num_traits` crate provides numeric traits for generic mathematics.
use num_traits::NumCast;
use num_traits::One;
use num_traits::Zero;

fn main() {
    // Create a variable with an initial value of zero.
    let mut x: i32 = Zero::zero();
    println!("Initial value of x: {x}");

    // Set x to one using the One trait
    x = One::one();
    println!("Value of x after setting to one: {x}");

    // Perform a type conversion using NumCast
    let y: f64 = NumCast::from(x).unwrap();
    println!("Value of y after converting from x: {y}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
