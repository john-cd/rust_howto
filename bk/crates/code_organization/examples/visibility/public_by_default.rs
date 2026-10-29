// TODO Reviewed ??-??-2026.

#![allow(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Enum variants and trait items are public by default; struct fields are not.

/// A module with public types and traits.
mod a_module {

    /// Public enum.
    pub enum Enum {
        // Public-by-default variant.
        /// A variant, public like its enum.
        Variant,
    }

    /// Public struct with a private field.
    pub struct PublicStruct {
        /// A private field.
        private_val: bool,
    }

    impl PublicStruct {
        /// Public associated function ("constructor").
        pub fn new(val: bool) -> Self {
            Self { private_val: val }
        }
    }

    /// Public struct with a public field.
    pub struct PublicStructPublicField {
        /// A public field.
        pub public_val: bool,
    }

    /// Public trait.
    pub trait Trait {
        /// An associated type, public like the trait.
        type Out; // Public-by-default associated type.

        // Visibility qualifiers are not permitted below:
        /// A method, public like the trait.
        fn method(&self) -> Self::Out;
    }
}

/// Uses the public items of the module.
fn main() {
    let _ = a_module::Enum::Variant;
    let _ = a_module::PublicStructPublicField { public_val: true };

    // We can't access `private_val`,
    // but we can use the public associated function to create the struct:
    // ERROR: let _ = a_module::PublicStruct { private_val: true };
    let _ = a_module::PublicStruct::new(false);
    println!(
        "public_by_default example: enum variants and trait items are public by default"
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the public by default example through its entry point.
    #[test]
    fn test_public_by_default_example_entry_point() {
        main();
    }
}
