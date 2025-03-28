// ANCHOR: example
fn main() {
    let x: f64 = 6.0;

    let a = x.tan();
    let b = x.sin() / x.cos();

    println!("a: {a}, b: {b}");
    assert_eq!(a, b);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
