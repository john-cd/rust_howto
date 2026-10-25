#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Turn a title into a URL-friendly slug.

use slug::slugify;

/// Converts titles to lowercase ASCII slugs.
pub fn run() {
    // Punctuation is dropped, spaces become hyphens, and accented characters
    // are transliterated to ASCII.
    assert_eq!(slugify("Hello, Wörld!"), "hello-world");
    assert_eq!(slugify("  Rust:  the   Book  "), "rust-the-book");
    assert_eq!(slugify("Crème brûlée"), "creme-brulee");
    println!("{}", slugify("Hello, Wörld!"));
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_succeeds() {
        run();
    }
}
