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
// [write](https://github.com/john-cd/rust_howto/issues/830)
