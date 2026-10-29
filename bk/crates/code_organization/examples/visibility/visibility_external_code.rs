// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! In the crate root:

// This function is available to external code using this crate as a dependency.
/// A function that external crates can call.
pub fn public_to_external_crates() -> &'static str {
    "public_to_external_crates"
}

// This module is public, so external crates may look inside of it,
// and access the inner public function:
/// A public module, so external crates can look inside it.
pub mod public_api {
    /// A function that external crates can call through the public module.
    pub fn also_public_to_external_crates() -> &'static str {
        "also_public_to_external_crates"
    }
}

// In contrast, this module is private, so an external crate cannot access it.
// It is accessible from this module and its descendants. Its public items are
// still restricted by the module's private boundary.
/// A private module, which external crates cannot access.
mod crate_helper_module {
    /// A function that can only be used within the crate.
    pub fn crate_helper() -> &'static str {
        "crate_helper"
    }
}

/// Calls the function of the private module.
fn main() {
    println!("{}", crate_helper_module::crate_helper());
    println!(
        "visibility_external_code example: public and private visibility in a crate"
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
    fn test_public_api() {
        assert_eq!(public_to_external_crates(), "public_to_external_crates");
        assert_eq!(
            public_api::also_public_to_external_crates(),
            "also_public_to_external_crates"
        );
    }

    #[test]
    fn test_crate_helper() {
        assert_eq!(crate_helper_module::crate_helper(), "crate_helper");
    }
}
