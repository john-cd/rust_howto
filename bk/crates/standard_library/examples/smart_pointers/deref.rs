// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Implementing `Deref` for a smart pointer.

use std::ops::Deref;

// Define a custom smart pointer.
/// A box that holds a value and dereferences to it.
struct MyBox<T>(T);

// Implement `Deref` for `MyBox`.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Dereferences a `MyBox` and relies on deref coercion.
fn main() {
    let wrapped = MyBox(String::from("Hello, Deref!"));

    // Even though `wrapped` is a `MyBox<String>`, we can call `len()`
    // as if it's a `String`, thanks to automatic deref coercion.
    println!("Length: {}", wrapped.len());

    // We can also explicitly dereference:
    println!("Uppercase: {}", (*wrapped).to_uppercase());
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the deref example through its entry point.
    #[test]
    fn test_deref_example_entry_point() {
        main();
    }
}
