// ANCHOR: example

/// This example demonstrates that the tangent of a number is equal to the sine
/// of the number divided by the cosine of the number.
fn main() {
    // Define a number.
    let x: f64 = 6.0;

    // Calculate the tangent of the number.
    let a = x.tan();
    // Calculate the sine of the number divided by the cosine of the number.
    let b = x.sin() / x.cos();

    println!("a: {a}, b: {b}");
    assert_eq!(a, b);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
