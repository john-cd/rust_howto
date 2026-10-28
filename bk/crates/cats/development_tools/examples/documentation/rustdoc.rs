// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Documenting a function, including documentation from an external file.

/// This is a doc comment.
/// Note the three slashes.
/// The first line is equivalent to the following:
#[doc = "This is a doc comment."]
fn documented_function() {
    println!("Function with doc comment.");
}

// Alternatively, we may use an external file.
// This is useful for including large amounts of documentation.
/// Demonstrates attaching an external Markdown file to an item's documentation.
#[cfg_attr(doc, doc = include_str!("../../../../README.md"))]
fn function_including_external_file_as_documentation() {}

/// Calls the documented functions.
fn main() {
    documented_function();
    function_including_external_file_as_documentation();
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the rustdoc example through its entry point.
    #[test]
    fn test_rustdoc_example_entry_point() {
        main();
    }
}
