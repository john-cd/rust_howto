// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! The newtype pattern: implementing an external trait on an external type.
//!
//! The orphan rule forbids implementing an external trait for an external type.
//! A newtype wraps the type in your own struct, which you own, so you can
//! implement the trait for it.

use std::fmt;

// // ERROR: only traits defined in the current crate can be implemented for
// // types defined outside of the crate:
// impl fmt::Display for Vec<String> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[{}]", self.join(", "))
//     }
// }

/// Tuple struct wrapping the type we want to add a non-local trait to.
///
/// This is the 'newtype pattern'.
struct Wrapper(Vec<String>);

/// Implement a (non-local) trait on the (local) `Wrapper` type:
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
// If you want the new type to have every method the inner type has,
// implement the `Deref` trait.

/// Prints a wrapped vector with its own `Display`.
fn main() {
    println!(
        "{}",
        Wrapper(vec!["example".to_owned(), "example 2".to_owned()])
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the newtype example through its entry point.
    #[test]
    fn test_newtype_example_entry_point() {
        main();
    }

    /// The wrapper displays its items between brackets.
    #[test]
    fn test_wrapper_display() {
        let w = Wrapper(vec!["example".to_owned(), "example 2".to_owned()]);
        assert_eq!(w.to_string(), "[example, example 2]");
        assert_eq!(Wrapper(vec![]).to_string(), "[]");
        assert_eq!(Wrapper(vec!["a".to_owned()]).to_string(), "[a]");
    }
}
