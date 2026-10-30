// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Fast and easy queue abstraction.
//!
//! Provides an abstraction over a queue. When the abstraction is used
//! there are these advantages:
//! - Fast
//! - `[Easy]`
//!
//! [Easy]: http://thatwaseasy.example.com

/// Prints a description of inner doc comments.
fn main() {
    println!(
        "//! ... are `inner` comments that apply to the containing module (or crate)."
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the rustdoc3 example through its entry point.
    #[test]
    fn test_rustdoc3_example_entry_point() {
        main();
    }

    // output-only: the example only prints a fixed explanatory sentence.
}
