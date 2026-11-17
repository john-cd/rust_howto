// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Documents functions and includes an external file in the module documentation.
//!
//! `rustdoc` builds HTML documentation from doc comments, with links between
//! items. Run `cargo doc --open`. Examples in the comments are tested, so the
//! documentation stays correct.

// Include the workspace README in the module documentation when building docs.
#![cfg_attr(doc, doc = include_str!("../../../../README.md"))]

/// This is a doc comment.
/// Note the three slashes.
/// The first line is equivalent to the following:
#[doc = "This is a doc comment."]
fn documented_function() {
    println!("Function with doc comment.");
}

/// Shows a short doc comment on a function with no implementation.
fn function_with_short_documentation() {}

/// Calls the documented functions.
fn main() {
    documented_function();
    function_with_short_documentation();
}
// ANCHOR_END: example

/// Calls the documented functions.
pub fn run() {
    main();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the rustdoc example through its entry point.
    #[test]
    fn test_rustdoc_example_entry_point() {
        main();
    }

    // output-only: the functions only print or carry documentation.

    /// Checks that documented functions are callable.
    #[test]
    fn documented_functions_are_callable() {
        documented_function();
        function_with_short_documentation();
    }
}
