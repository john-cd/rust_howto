//! Organizing a library with reexports and a prelude.
//!
//! The modules are declared inline so that the example fits in one file. In a
//! real crate, each would live in its own file.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: reexports

// ANCHOR: private_modules
/// A module that keeps its submodules private and reexports the items of
/// interest, which flattens the module hierarchy.
mod module1 {
    // Private submodules.

    /// Demonstrates the `submodule1` child module in this module layout example.
    mod submodule1 {

        /// A public function in a private submodule.
        pub fn public_function() -> &'static str {
            "public_function"
        }
    }

    /// Demonstrates the `submodule2` child module in this module layout example.
    mod submodule2 {

        /// A public struct in a private submodule.
        #[derive(Debug, Default)]
        pub struct Struct1;
    }

    // Reexport a public function and struct.
    pub use submodule1::public_function;
    pub use submodule2::Struct1;
}
// ANCHOR_END: private_modules

// The function and struct appear as if they were defined in `module1`.
use module1::Struct1;
use module1::public_function;
// ANCHOR_END: reexports

// ANCHOR: prelude

/// A library with a prelude that reexports its most commonly used items.
mod my_library {

    /// Provides the `prelude` implementation used by this module.
    pub mod prelude {
        // Reexport commonly used types, traits, or functions:
        pub use super::a_module::SuperCommonType;
        pub use super::a_module::super_common_function;
    }

    /// Provides the `a_module` implementation used by this module.
    pub mod a_module {

        /// A function that most clients need.
        pub fn super_common_function() -> u32 {
            42
        }

        /// A type that most clients need.
        #[derive(Debug, Default)]
        pub struct SuperCommonType {
            /// Value stored by this wrapper.
            pub value: u32,
        }
    }
}

// In the client, a single glob import brings the common items into scope:
use my_library::prelude::*;
// ANCHOR_END: prelude

/// Uses the reexported and prelude items.
// ANCHOR: example
fn main() {
    println!("{}", public_function());
    let s1 = Struct1;
    println!("{s1:?}");

    let common = SuperCommonType {
        value: super_common_function(),
    };
    println!("{common:?} {}", common.value);
}

// ANCHOR_END: example

/// Runs the book example.
pub fn run() {
    let entry_point = main;
    entry_point();
}
/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Imports public re-exports through the crate root.
    #[test]
    fn test_reexports_are_reachable() {
        assert_eq!(public_function(), "public_function");
        let s1 = Struct1;
        assert_eq!(format!("{s1:?}"), "Struct1");
    }

    /// Verifies prelude items are reachable.
    #[test]
    fn test_prelude_items_are_reachable() {
        assert_eq!(super_common_function(), 42);
        assert_eq!(SuperCommonType::default().value, 0);
    }
}

/// Provides private main tests behavior for the reexports and prelude component.
#[cfg(test)]
mod private_main_tests {
    use super::main;

    /// Checks that private main succeeds.
    #[test]
    fn private_main_succeeds() {
        let entry_point = main;
        entry_point();
    }
}
