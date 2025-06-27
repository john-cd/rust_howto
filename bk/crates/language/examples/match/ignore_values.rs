#![allow(dead_code)]
// ANCHOR: example
fn main() {
    // `_unused` is bound to the value `42`, but the compiler does not complain
    // if it is not used later.
    let _unused = 42;

    let numbers = (2, 4, 8, 16, 32);

    // Use `..` to ignore e.g. the middle elements:
    let (first, .., last) = numbers;
    println!("Some numbers: first = {first}, last = {last}");

    // Use `_` to ignore a value completely:
    fn foo(_: i32, y: i32) {
        // The first parameter is ignored.
        println!("y is: {y}");
    }

    foo(42, 43);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
