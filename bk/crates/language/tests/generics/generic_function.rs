// ANCHOR: example
use std::fmt::Debug;

// Define a generic function `print_value` that takes one argument `value` of
// any type `T`. The `T: std::fmt::Debug` part is a trait bound, meaning `T`
// must implement the `Debug` trait, so that it can be printed using the `{:?}`
// format specifier.
fn print_value<T: Debug>(value: T) {
    println!("The value is: {:?}", value);
}

fn main() {
    // Call the generic function with an integer.
    print_value(10);
    // Call the generic function with a floating-point number.
    print_value(2.14);
    // Call the generic function with a string slice.
    print_value("hello");
    // Call the generic function with a custom struct that implements `Debug`.
    print_value(MyStruct { value: 42 });
}

#[derive(Debug)]
struct MyStruct {
    value: i32,
}

// ANCHOR_END: example

#[test]
fn test() {
    main();
}
