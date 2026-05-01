#![allow(dead_code)]
// ANCHOR: example
/// Calculates the hypotenuse of a right triangle given an angle and the length
/// of the side opposite to it.
fn main() {
    // The angle in radians.
    let angle: f64 = 2.0;
    // The length of the side opposite to the angle.
    let side_length = 80.0;

    // Calculate the hypotenuse using the sine function.
    let hypotenuse = side_length / angle.sin();

    println!("Hypotenuse: {hypotenuse}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
