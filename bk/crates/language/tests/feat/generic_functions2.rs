// ANCHOR: example
use std::fmt::Display;

// This is a generic function that can work with types that may not have a
// known size at compile time.
// It requires that the type `T` implements the `Display` trait, so it can
// be printed.
fn generic<T: ?Sized + Display>(t: &T) {
    // By default, generic functions will work only on types that have a
    // known size at compile time. Use `?Sized` to relax that rule.
    println!("{}", t);
}

fn main() {
    let s = String::from("hello");
    generic(&s[..]);
    generic(&s);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
