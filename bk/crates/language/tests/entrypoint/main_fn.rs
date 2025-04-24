// ANCHOR: example
// `main` takes no parameters and implicitly returns the "unit type" (), similar
// to 'void' in C/C++/Java.
fn main() {
    println!("Hello, world!");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
