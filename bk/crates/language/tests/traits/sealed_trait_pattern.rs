#![allow(unused)]
// ANCHOR_END: example
//! Example of a sealed trait, preventing external implementations.

mod a_module {
    #[allow(private_bounds)]
    pub trait MySealedTrait: Sealed {
        // Public method accessible to the user.
        // Associated items in a `pub` Trait are public by default.
        fn some_public_method(&self);
    }

    // A private trait that is only accessible from `a_module`.
    trait Sealed {}

    // Implement the trait for specified types only, here `usize`:
    impl MySealedTrait for usize {
        // Trait items always share the visibility of their trait.
        fn some_public_method(&self) {}
    }

    // The supertrait must be implemented as well:
    impl Sealed for usize {}
}

fn main() {
    use a_module::MySealedTrait;

    let a: usize = 42;
    a.some_public_method();

    // We can't implement the trait on another type here:
    // ERROR: trait `Sealed` is private
    // impl a_module::Sealed for i32 {}

    let _b: i32 = 43;
    // ERROR: _b.some_public_method(); // no method named `some_public_method`
    // found for type `i32` in the current scope
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
