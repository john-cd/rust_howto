#![allow(unused_imports)]
// TODO Reviewed ??-??-2026.
#![allow(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! The `pub` keyword on functions, types, traits and modules.

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

/// Uses the public items of the module `m`.
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
    println!("pub_keyword example: public items accessible, private items not");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the pub keyword example through its entry point.
    #[test]
    fn test_pub_keyword_example_entry_point() {
        main();
    }
}
