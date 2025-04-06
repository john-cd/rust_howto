// ANCHOR: example

/// Creates complex numbers with integer and floating-point components.
fn main() {
    // Create a complex number with integer components.
    let complex_integer = num::complex::Complex::new(10, 20);

    // Create a complex number with floating-point components.
    let complex_float = num::complex::Complex::new(10.1, 20.1);

    println!("Complex integer: {}", complex_integer);
    println!("Complex float: {}", complex_float);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
