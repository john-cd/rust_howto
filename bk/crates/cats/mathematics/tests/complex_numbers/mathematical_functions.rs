// ANCHOR: example
use std::f64::consts::PI;

use num::complex::Complex;

/// This example shows how to use the `exp` function on a complex number.
fn main() {
    let x = Complex::new(0.0, 2.0 * PI);

    println!("e^(2i * pi) = {}", x.exp()); // =~1
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
