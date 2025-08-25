#![allow(dead_code)]
#![allow(unused_imports)]
// ANCHOR: example
/// Private module.
mod m {
    /// Private function.
    fn private_function() {}

    /// Public function.
    pub fn public_function() {}

    /// Public unit-like struct.
    pub struct Struct;

    impl Struct {
        /// Public method.
        pub fn method(&self) {}
    }

    /// Public trait.
    pub trait Trait {}

    /// Trait implementation.
    /// `pub` is not allowed here.
    impl Trait for Struct {}

    /// Private submodule.
    mod private_module {}

    /// Public submodule.
    pub mod public_module {}
}

fn main() {
    // Access the public function within module `m`:
    m::public_function();
    // ERROR: m::private_function();

    // Access the public struct and trait.
    let _t: &dyn m::Trait = &m::Struct;

    // Access the public method.
    m::Struct.method();
    // Equivalent to `m::Struct::method(&m::Struct);`.

    // Bring the public submodule into scope.
    use m::public_module;
    // ERROR: use m::private_module;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
