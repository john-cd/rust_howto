//! Restricting visibility to the current crate with `pub(crate)`.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// ANCHOR: example
// In e.g. `lib.rs`:
/// A module with a `pub(crate)` function.
mod a_module {
    // Conservative use of `pub(crate)`:
    // Even if `a_module` is accidentally made fully public,
    // this function will remain visible only to this crate.
    /// A function visible only in the current crate.
    pub(crate) fn visible_in_crate_only() {}
}

// We can also explictly mark the top module as `pub(crate)`:
/// A `pub(crate)` module.
pub(crate) mod b_module {
    /// A function visible only in the current crate, since its module is.
    pub fn visible_in_crate_only() {}
}

/// Calls the functions that are visible in the crate.
fn main() {
    a_module::visible_in_crate_only();
    b_module::visible_in_crate_only();
    println!(
        "visibility_pub_crate example: pub(crate) restricts visibility to the current crate"
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
