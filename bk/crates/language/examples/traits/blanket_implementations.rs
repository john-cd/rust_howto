#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A blanket implementation of a trait for all types that implement another.

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

    #[test]
    fn test_main() {
        main();
    }
}
