#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Buildable placeholder example.
//!
//! Original placeholder comments are preserved below as comments.

/// Prints a message: the example is a placeholder.
fn main() {
    println!("Placeholder example.");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the render example through its entry point.
    #[test]
    fn test_render_example_entry_point() {
        main();
    }
}
// [write](https://github.com/john-cd/rust_howto/issues/830)
