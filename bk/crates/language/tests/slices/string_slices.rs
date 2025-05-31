// ANCHOR_END: example
//! String slice examples.

fn main() {
    // A string literal:
    let string_literal: &'static str = "hello world";

    // An owned `String` created from the literal:
    let s: String = String::from(string_literal);

    // String slices from the `String`:
    let hello: &str = &s[0..5]; // or &s[..5];
    let world: &str = &s[6..11]; // or &s[6..];
    println!("{}", hello);
    println!("{}", world);

    // Take a slice of the entire `String`:
    let whole_slice = &s[..];

    // A string slice is a view into the `String`:
    // The inner pointers of the slice and the string are equal.
    assert!(std::ptr::eq(whole_slice.as_ptr(), s.as_ptr()));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
