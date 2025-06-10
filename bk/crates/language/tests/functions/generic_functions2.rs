#![allow(dead_code)]
// ANCHOR: example
use std::fmt::Debug;
use std::fmt::Display;

/// Generic functions can accept trait bounds.
// Here we define a generic function `print_value` that takes one argument
// `value` of any type `T`. The `T: std::fmt::Debug` part is a trait bound,
// meaning `T` must implement the `Debug` trait, so that it can be printed using
// the `{:?}` format specifier.
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
    // Call the generic function with an integer.
    print_value(10);

    // Call the generic function with a floating-point number.
    print_value(2.14);

    // Call the generic function with a string slice.
    print_value("hello");

    // Call the generic function with a custom struct that implements `Debug`.
    #[allow(dead_code)]
    #[derive(Debug)]
    struct MyStruct {
        value: i32,
    }
    print_value(MyStruct { value: 42 });

    let s = String::from("hello");
    generic(&s[..]);
    generic(&s);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
