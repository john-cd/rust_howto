use std::f64::consts::PI;

use num::complex::Complex;

fn main() {
    let x = Complex::new(0.0, 2.0 * PI);

    println!("e^(2i * pi) = {}", x.exp()); // =~1
}
