#![allow(dead_code)]
// ANCHOR: example
//! In the crate root:

// This function is available to external code using this crate as a dependency.
pub fn public_to_external_crates() {}

// This module is public, so external crates may look inside of it,
// and access the inner public function:
pub mod public_api {
    pub fn also_public_to_external_crates() {}
}

// In constrast, this module is private, meaning that no external crate can
// access this module. It is however accessible from the current module and any
// descendants. If it is private at the root of this current crate, any module
// in the crate may access any publicly visible item in this module.
mod crate_helper_module {
    pub fn crate_helper() {}
}

fn main() {
    crate_helper_module::crate_helper();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
