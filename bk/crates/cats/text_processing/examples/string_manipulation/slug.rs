// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Turn a title into a URL-friendly slug.
//!
//! A slug is the readable part of a URL derived from a title: lowercase, words
//! joined by hyphens, accents removed (for example `my-first-post`). It makes
//! addresses friendly to people and search engines.

use slug::slugify;
/// Runs the example entry point.
///
/// Converts titles to lowercase ASCII slugs.
pub fn example_body() {
    // Punctuation is dropped, spaces become hyphens, and accented characters
    // are transliterated to ASCII.
    assert_eq!(slugify("Hello, Wörld!"), "hello-world");
    assert_eq!(slugify("  Rust:  the   Book  "), "rust-the-book");
    assert_eq!(slugify("Crème brûlée"), "creme-brulee");
    println!("{}", slugify("Hello, Wörld!"));
}
// ANCHOR_END: example

/// Runs the example function.
pub fn run() {
    example_body();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_succeeds() {
        run();
    }
}
