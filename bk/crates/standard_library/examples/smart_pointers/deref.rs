#![allow(dead_code)]
// ANCHOR: example
use std::ops::Deref;

// Define a custom smart pointer.
struct MyBox<T>(T);

// Implement `Deref` for `MyBox`.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let wrapped = MyBox(String::from("Hello, Deref!"));

    // Even though `wrapped` is a `MyBox<String>`, we can call `len()`
    // as if it's a `String`, thanks to automatic deref coercion.
    println!("Length: {}", wrapped.len());

    // We can also explicitly dereference:
    println!("Uppercase: {}", (*wrapped).to_uppercase());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
