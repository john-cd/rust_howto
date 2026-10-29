// TODO Reviewed ??-??-2026.

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

        /// Public method, which can read the private field.
        pub fn val(&self) -> bool {
            self.private_val
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

    impl Trait for PublicStruct {
        type Out = bool;

        fn method(&self) -> bool {
            self.private_val
        }
    }
}

/// Uses the public items of the module.
fn main() {
    let _ = a_module::Enum::Variant;
    let s = a_module::PublicStructPublicField { public_val: true };
    println!("public_val = {}", s.public_val);

    // We can't access `private_val`,
    // but we can use the public associated function to create the struct:
    // ERROR: let _ = a_module::PublicStruct { private_val: true };
    let p = a_module::PublicStruct::new(false);
    println!("private_val = {}", p.val());

    // Use the public trait:
    use a_module::Trait;
    println!("method = {}", p.method());
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

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_enum_variants_and_pub_fields_are_public() {
        let e = a_module::Enum::Variant;
        assert!(matches!(e, a_module::Enum::Variant));
        let s = a_module::PublicStructPublicField { public_val: true };
        assert!(s.public_val);
    }

    #[test]
    fn test_trait_items_are_public() {
        struct Doubler;
        impl a_module::Trait for Doubler {
            type Out = u8;

            fn method(&self) -> u8 {
                2
            }
        }
        assert_eq!(a_module::Trait::method(&Doubler), 2);
    }

    #[test]
    fn test_constructor_with_private_field() {
        let _ = a_module::PublicStruct::new(true);
    }
}
