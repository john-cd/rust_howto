#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates the `doc-comment` crate for attaching
//! documentation text to a Rust item.

use doc_comment::doc_comment;

doc_comment!(
    concat!(
        "This function is documented by the `doc_comment!` macro.",
        "\n\n",
        "The macro is useful when documentation text is generated from a string."
    ),
    /// Prints a confirmation when the documentation example runs.
    pub fn documented_function() {
        println!("doc-comment example executed");
    }
);

/// Runs the doc comment example.
fn main() {
    documented_function();
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
