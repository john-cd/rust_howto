//! Rewriting tokens with a procedural macro built on `proc-macro2`.
//!
//! TODO Reviewed ??-??-2026

#![allow(unused)]

// ANCHOR: example
// Import our custom procedural macro that was built using `proc-macro2`.
use proc_macros::replace_ident;

/// Uses `replace_ident!` to rename an identifier.
fn main() {
    // The `replace_ident` macro replaces any occurrence of `old_ident`
    // with `new_ident` inside its token stream.
    replace_ident! {
        let old_ident = "Hello, proc-macro2!";
        println!("{new_ident}");
        assert_eq!(new_ident, "Hello, proc-macro2!");
    }
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
