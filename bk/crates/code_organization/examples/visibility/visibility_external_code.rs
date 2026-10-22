#![allow(dead_code)]
// ANCHOR: example
//! In the crate root:
//!
//! TODO Reviewed ??-??-2026

// This function is available to external code using this crate as a dependency.
/// A function that external crates can call.
pub fn public_to_external_crates() {}

// This module is public, so external crates may look inside of it,
// and access the inner public function:
/// A public module, so external crates can look inside it.
pub mod public_api {
    /// A function that external crates can call through the public module.
    pub fn also_public_to_external_crates() {}
}

// In constrast, this module is private, meaning that no external crate can
// access this module. It is however accessible from the current module and any
// descendants. If it is private at the root of this current crate, any module
// in the crate may access any publicly visible item in this module.
/// A private module, which external crates cannot access.
mod crate_helper_module {
    /// A function that can only be used within the crate.
    pub fn crate_helper() {}
}

/// Calls the function of the private module.
fn main() {
    crate_helper_module::crate_helper();
    println!(
        "visibility_external_code example: public and private visibility in a crate"
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
