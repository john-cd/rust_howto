#![allow(dead_code)]
// ANCHOR: example
mod a_module {

    /// Public enum.
    pub enum Enum {
        // Public-by-default variant.
        Variant,
    }

    /// Public struct with a private field.
    pub struct PublicStruct {
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
        pub public_val: bool,
    }

    /// Public trait.
    pub trait Trait {
        type Out; // Public-by-default associated type.
        // Visibility qualifiers are not permitted below:
        fn method(&self) -> Self::Out;
    }
}

fn main() {
    let _ = a_module::Enum::Variant;
    let _ = a_module::PublicStructPublicField { public_val: true };

    // As we will see below, we can't access `private_val`,
    // but we can use its public associated function to create the struct:
    // ERROR: let _ = a_module::PublicStruct { private_val: true };
    let _ = a_module::PublicStruct::new(false);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
