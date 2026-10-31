// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A blanket implementation of a trait for all types that implement another.
//!
//! A blanket implementation implements a trait for every type that meets a
//! bound, for example `impl<T: Display> MyTrait for T`. The standard library
//! does this: `ToString` exists for everything that implements `Display`.

/// A module that defines a trait and its blanket implementation.
mod my {
    use std::fmt::Display;

    /// A trait implemented for every type that implements `Display`.
    pub trait VeryUsefulTrait {
        /// Returns a description of `self`.
        fn my_method(&self) -> String;
    }

    // Blanket implementation for ALL types that implement `Display`:
    impl<T: Display> VeryUsefulTrait for T {
        fn my_method(&self) -> String {
            format!("{self}")
        }
    }
}

/// Calls the blanket implementation on a string.
fn main() {
    // Don't forget to bring the trait in scope:
    use my::VeryUsefulTrait as _;

    let s = String::from("Hello, world!");

    // There is no need for an individual implementation
    // of `VeryUsefulTrait` for `String`, since it implements `Display`:
    println!("{}", s.my_method());
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the blanket implementations example through its entry point.
    #[test]
    fn test_blanket_implementations_example_entry_point() {
        main();
    }

    /// Every `Display` type gets the method.
    #[test]
    fn test_blanket_impl() {
        use my::VeryUsefulTrait as _;
        assert_eq!("Hello".my_method(), "Hello");
        assert_eq!(42.my_method(), "42");
        assert_eq!(1.5_f64.my_method(), "1.5");
        assert_eq!('c'.my_method(), "c");
        assert_eq!(String::new().my_method(), "");
    }
}
