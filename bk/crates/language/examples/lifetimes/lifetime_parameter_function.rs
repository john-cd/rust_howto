#![allow(dead_code)]
// ANCHOR: example
/// This function has a lifetime parameter `'a`.
/// The lifetime parameter can be used by references within the function
/// signature, return type, or body.
///
/// In this example, the function takes two string slices, `x` and `y`, both
/// with the same lifetime `'a`. It returns a string slice that is the longer of
/// the two, also with the lifetime `'a`.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // We define two variables, one with a scope / lifetime
    // shorter than the other.
    let x = String::from("looooooong");
    let l;
    {
        let y = String::from("short");
        l = longest(&x, &y);
        println!("Longest: {l}");
    } // y is dropped here.
    // println!("Longest: {l}"); // ERROR: borrowed value does not live long
    // enough.
    println!("x: {x}");
    // x is valid until this line.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
