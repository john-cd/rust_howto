//! Accessing private items from the same module and its children.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]
#![allow(unused_imports)]

// ANCHOR: example
/// A function private to the parent module.
fn private_in_parent() {}

/// A private module.
mod private_module {
    /// A function private to `private_module`.
    fn also_private() {
        // `private_in_parent` is accessible,
        // since it is in a parent module.
        super::private_in_parent();
    }

    /// A private submodule.
    mod submodule {}
}

/// Calls the private items that are accessible from here.
fn main() {
    // `private_in_parent` is accessible, since it is
    // in the same module than `main`.
    private_in_parent();

    // `private_module` is accessible as well,
    // because it is also in the same module...
    use private_module as alias;

    // ...but its private contents are not!
    // ERROR: private_module::also_private();
    // ERROR: alias::also_private();
    // ERROR: use private_module::submodule;
    println!(
        "private_access example: private items accessed only from parent/same module"
    );
}

// ANCHOR_END: example

/// Runs the example.
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
}
