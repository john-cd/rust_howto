#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates the `doc-comment` crate for generating documentation
//! comments from string literals.

use doc_comment::doc_comment;

doc_comment! {
    """
    This function is documented by the `doc_comment!` macro.

    It is useful when documentation needs to be generated programmatically,
    or when examples should be kept near the implementation without writing
    raw `///` comments.
    """
    pub fn documented_function() {
        println!("doc-comment example executed");
    }
}

fn main() {
    documented_function();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
