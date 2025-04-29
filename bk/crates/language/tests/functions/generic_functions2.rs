// ANCHOR: example
use std::fmt::Debug;
use std::fmt::Display;

/// Generic functions can accept trait bounds.
///
/// This function works for any type `T` that implements the `Debug` trait.
fn print_value<T: Debug>(value: T) {
    println!("The value is: {:?}", value); // `{:?}` can be used, because `value` implements `Debug`.
}

/// Type constraints can be written in a separate `where` clause for clarity.
/// You can also combine multiple trait bounds with `+`.
fn print_value2<T>(value: T)
where
    T: Debug + Display,
{
    println!("Compare {} and {:?}", value, value);
}

/// All type parameters have an implicit bound of `Sized`.
/// The special syntax `?Sized` can be used to remove this bound if it is not
/// appropriate.
///
/// This function can work with types that may not have a known size at compile
/// time.
fn generic<T: ?Sized + Display>(t: &T) {
    println!("{}", t);
}

fn main() {
    print_value(1);
    print_value2("2");
    let s = String::from("hello");
    generic(&s[..]);
    generic(&s);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
