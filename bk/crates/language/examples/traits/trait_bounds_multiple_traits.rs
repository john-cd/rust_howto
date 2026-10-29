// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Requiring several traits with `+`.

use std::clone::Clone;
use std::fmt::Debug;

/// This function takes a reference to a type that must implement both the
/// `Debug` and `Clone` traits. Note the `+`
fn func<T: Debug + Clone>(t: &T) {
    println!("{:?}", t.clone());
    // We can call `clone` because `T` implements `Clone`.
    // We can also print `t` with `:?` because it implements `Debug`.
}

/// It may be equivalently be written in a `where` clause:
fn func2<T>(t: &T)
where
    T: Debug + Clone,
{
    println!("{:?}", t.clone());
}

/// You can also use the similar `impl Trait` syntax.
/// `impl Trait` is an opaque type that implements the specified traits.
/// Parentheses are necessary.
fn func3(item: &(impl Debug + Clone)) {
    println!("{:?}", item.clone());
}

/// A type that implements `Debug` and `Clone`.
#[derive(Debug, Clone)]
struct S;

/// Calls the functions that require both traits.
fn main() {
    let s = S;
    // Since `S` implements both `Debug` and `Clone`, we can call the above
    // functions:
    func(&s);
    func2(&s);
    func3(&s);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the trait bounds multiple traits example through its entry point.
    #[test]
    fn test_trait_bounds_multiple_traits_example_entry_point() {
        main();
    }

    // Output-only example: the functions only print a clone.
    /// The functions accept any `Debug + Clone` type.
    #[test]
    fn test_other_types() {
        func(&1);
        func2(&"x");
        func3(&vec![1, 2]);
    }
}
