// TODO Reviewed ??-??-2026.
// ANCHOR: example
//! Which items are accessible through nested public and private modules.

/// A private module.
mod a_module {
    /// A private function.
    fn private() {}

    /// A public function.
    pub fn public() -> &'static str {
        private();
        private_nested_module::public();
        "a_module::public"
    }

    /// A public nested module.
    pub mod public_nested_module {
        /// A private function.
        fn private() {}

        /// A public function.
        pub fn public() -> &'static str {
            private();
            "public_nested_module::public"
        }
    }

    /// A private nested module.
    mod private_nested_module {
        /// A private function.
        fn private() {}

        /// A public function.
        pub fn public() {
            private();
        }
    }
}

/// Calls the accessible items.
fn main() {
    // For an item to be accessible, all elements in the paths
    // must be accessible from the current module.
    println!("{}", a_module::public());

    // Here, `a_module` is accessible, despite being private,
    // because it is in the same module as `main`.

    // No access, if one or more segments in the path are inaccessible.
    // Here, `private` can't be reached from a parent.
    // ERROR: a_module::private();

    println!("{}", a_module::public_nested_module::public());
    // ERROR: a_module::public_nested_module::private();
    // ERROR: a_module::private_nested_module::private();
    // ERROR: a_module::private_nested_module::public();
    call_sibling();
    println!(
        "public_access example: public items in public modules are accessible"
    );
}

// The same applies when calling from a sibling module.
/// A module that is a sibling of `a_module`.
mod another_module {
    /// Calls the items accessible from this module.
    pub fn call() {
        // `super` refers to the parent and is therefore accessible
        // from the child.
        super::a_module::public();
        // ERROR: super::a_module::private();
        super::a_module::public_nested_module::public();
        // ERROR: super::a_module::private_nested_module::public();
    }
}

/// Calls the function of the sibling module.
fn call_sibling() {
    another_module::call();
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
    fn test_public_items_in_public_modules() {
        assert_eq!(a_module::public(), "a_module::public");
        assert_eq!(
            a_module::public_nested_module::public(),
            "public_nested_module::public"
        );
    }
}
