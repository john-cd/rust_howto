// TODO Reviewed ??-??-2026.
// ANCHOR: example
//! The `pub` keyword on functions, types, traits and modules.

/// Private module.
mod m {
    /// Private function.
    fn private_function() {}

    /// Public function.
    pub fn public_function() -> &'static str {
        private_function(); // A private function is callable from the same module.
        "public_function"
    }

    /// Public unit-like struct.
    pub struct Struct;

    impl Struct {
        /// Public method.
        pub fn method(&self) -> &'static str {
            "method"
        }
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
    println!("{}", m::public_function());
    // ERROR: m::private_function();

    // Access the public struct and trait.
    let _t: &dyn m::Trait = &m::Struct;

    // Access the public method.
    println!("{}", m::Struct.method());
    // Equivalent to `m::Struct::method(&m::Struct);`.

    // Bring the public submodule into scope.
    #[expect(
        unused_imports,
        reason = "Demonstrates that public submodules can be imported."
    )]
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

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_public_items_accessible() {
        assert_eq!(m::public_function(), "public_function");
        assert_eq!(m::Struct.method(), "method");
        // The trait is implemented for the public struct.
        let _t: &dyn m::Trait = &m::Struct;
    }
}
