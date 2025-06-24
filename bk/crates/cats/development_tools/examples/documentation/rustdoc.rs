#![allow(dead_code)]
// ANCHOR: example
/// This is a doc comment.
/// Note the three slashes.
/// The first line is equivalent to the following:
#[doc = "This is a doc comment."]
fn documented_function() {
    println!("Function with doc comment.");
}

// Alternatively, you may use an external file.
// This is useful for including large amounts of documentation.
#[doc = include_str!("../../../../README.md")]
fn function_including_external_file_as_documentation() {}

fn main() {
    documented_function();
    function_including_external_file_as_documentation();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
