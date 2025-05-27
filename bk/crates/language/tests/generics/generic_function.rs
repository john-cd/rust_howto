// ANCHOR: example
use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Debug)]
struct MyStruct {
    value: i32,
}

// Define a generic function `print_value` that takes one argument `value` of
// any type `T`. The `T: std::fmt::Debug` part is a trait bound, meaning `T`
// must implement the `Debug` trait, so that it can be printed using the `{:?}`
// format specifier.
fn print_value<T: Debug>(value: T) {
    println!("The value is: {:?}", value);
}

/// Function with an explicit lifetime parameter.
/// `x` and `y` are string slices that live at least as long as the lifetime 'a.
/// The reference returned by the longest function will have the same lifetime
/// as the lifetime 'a.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
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

    // The compiler infers the concrete lifetime for 'a at the call site,
    // choosing the shorter of the two input lifetimes.
    let result = longest("abcd", "xyz");
    println!("The longest string is {}", result);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
