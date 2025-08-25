#![allow(dead_code)]
// ANCHOR: example
/// Sort a vector of floats.
fn main() {
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];

    // ERROR: vec.sort();

    // The vector does not contain `f64::NAN`,
    // so we can sort it using `partial_cmp` safely:
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);

    println!("{vec:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
