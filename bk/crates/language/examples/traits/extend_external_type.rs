#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Adding a method to an external type with a trait.

/// A trait for types that can be hashed to a `u64`.
trait MyHash {
    /// Returns a hash of the object.
    fn myhash(&self) -> u64;
}

/// Implement the `MyHash` trait for preexisting type `i64`,
/// which is defined in the standard library.
impl MyHash for i64 {
    fn myhash(&self) -> u64 {
        *self as u64
    }
}

/// Calls the method that the trait adds to `i64`.
fn main() {
    let x = 1i64;
    // Since `i64` implements the `MyHash` trait, we can call its method.
    println!("{}", x.myhash());
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
