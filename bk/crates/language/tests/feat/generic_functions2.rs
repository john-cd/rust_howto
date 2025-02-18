// ANCHOR: example
use std::fmt::Display;

fn generic<T: ?Sized + Display>(t: &T) {
    // By default, generic functions will work only on types that have a
    // known size at compile time. Use `?Sized` to relax that rule.
    // `t` must be some kind of (smart) pointer: &, `Rc`, `Box`...
    println!("{}", t);
}

fn main() {
    let s = String::from("hello");
    generic(&s[..]);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
