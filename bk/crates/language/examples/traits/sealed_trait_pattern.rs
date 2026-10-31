// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example of a sealed trait, preventing external implementations.

/// A module that exposes a trait that cannot be implemented outside of it.
mod a_module {
    /// A trait that can be used, but not implemented, outside of the module.
    #[allow(private_bounds)]
    pub trait MySealedTrait: Sealed {
        // Public method accessible to the user.
        // Associated items in a `pub` Trait are public by default.
        /// A method that users of the trait can call.
        fn some_public_method(&self);
    }

    // A private trait that is only accessible from `a_module`.
    /// A private trait that seals `MySealedTrait`.
    trait Sealed {}

    // Implement the trait for specified types only, here `usize`:
    impl MySealedTrait for usize {
        // Trait items always share the visibility of their trait.
        fn some_public_method(&self) {}
    }

    // The supertrait must be implemented as well:
    impl Sealed for usize {}
}

/// Calls a method of the sealed trait.
fn main() {
    use a_module::MySealedTrait as _;

    let a: usize = 42;
    a.some_public_method();

    // We can't implement the trait on another type here:
    // ERROR: trait `Sealed` is private
    // impl a_module::Sealed for i32 {}

    let _b: i32 = 43;
    // ERROR: _b.some_public_method(); // no method named `some_public_method`
    // found for type `i32` in the current scope
    println!("Sealed trait pattern: called some_public_method on {a}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the sealed trait pattern example through its entry point.
    #[test]
    fn test_sealed_trait_pattern_example_entry_point() {
        main();
    }

    /// Only the sealed implementors can use the method.
    #[test]
    fn test_sealed_trait_method() {
        use a_module::MySealedTrait as _;
        42_usize.some_public_method();
        0_usize.some_public_method();
    }

    // Output-only example: the sealed method has an empty body.
}
