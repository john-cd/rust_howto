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

    #[test]
    fn test_deref_example_entry_point() {
        main();
    }

    #[test]
    fn test_auto_deref_calls_methods_of_the_target() {
        let wrapped = MyBox(String::from("Hello, Deref!"));
        assert_eq!(wrapped.len(), 13);
        assert_eq!((*wrapped).to_uppercase(), "HELLO, DEREF!");
    }

    #[test]
    fn test_deref_coercion_to_str() {
        fn takes_str(s: &str) -> usize {
            s.len()
        }
        let wrapped = MyBox(String::from("abc"));
        // `&MyBox<String>` -> `&String` -> `&str`.
        assert_eq!(takes_str(&wrapped), 3);
        assert_eq!(takes_str(&MyBox(String::new())), 0);
    }

    #[test]
    fn test_explicit_deref() {
        let b = MyBox(5);
        assert_eq!(*b, 5);
        let as_ref: &i32 = &b;
        assert_eq!(as_ref, &5);
    }
}
