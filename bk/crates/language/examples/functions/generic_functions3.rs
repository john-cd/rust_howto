#![allow(dead_code)]
// ANCHOR: example
/// Function with an explicit lifetime parameter.
///
/// `x` and `y` are string slices that live at least as long as the lifetime
/// `'a`. The reference returned by the longest function will have the same
/// lifetime as the lifetime `'a`.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // The compiler infers the concrete lifetime for `'a` at the call site,
    // choosing the shorter of the two input lifetimes.
    let result = longest("abcd", "xyz");
    println!("The longest string is {}", result);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
