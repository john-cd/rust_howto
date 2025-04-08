// ANCHOR: example

/// This example demonstrates how to use `Cow` with `String` and `Deref`.
/// It shows that you can use `Cow` to borrow a string slice and then
/// use `deref()` to get a `&str` from it.
fn main() {
    use std::ops::Deref;
    let mut my_string = String::new();
    let example = std::borrow::Cow::from("example");

    my_string.push_str(example.deref());
    println!("{}", my_string);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
