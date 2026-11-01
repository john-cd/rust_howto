// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates the `doc-comment` crate for attaching
//! documentation text to a Rust item.
//!
//! Doc comments (`///` for items, `//!` for the enclosing module or crate)
//! become the generated documentation. Code blocks in them are compiled and run
//! as tests by `cargo test`, so examples cannot go stale.

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

pub fn run() {
    // Same body as `main`; calling `main` trips `clippy::main_recursion`.
    documented_function();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the doc comment example through its entry point.
    #[test]
    fn test_doc_comment_example_entry_point() {
        main();
    }

    #[test]
    fn documented_function_is_callable() {
        // The macro only attaches documentation; the body is left unchanged.
        documented_function();
    }

    // output-only: the function only prints a message.
}
