// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Comparing UTF-8 bytes, Unicode scalar values, and grapheme clusters.
//!
//! Unicode text has three levels: bytes (UTF-8 storage), scalar values (`char`)
//! and grapheme clusters (what a user sees as one character). An emoji with
//! skin tone or an accented letter can be several `char`s but one grapheme, so
//! counting `char`s is not counting what people see.

use unicode_segmentation::UnicodeSegmentation;

/// Prints grapheme clusters and compares their count with other string lengths.
fn main() {
    let text = "e\u{301} 🇫🇷";
    let graphemes =
        UnicodeSegmentation::graphemes(text, true).collect::<Vec<_>>();

    println!("UTF-8 bytes: {}", text.len());
    println!("Unicode scalar values: {}", text.chars().count());
    println!("Grapheme clusters: {graphemes:?}");

    assert_eq!(graphemes, ["e\u{301}", " ", "🇫🇷"]);
    assert_ne!("é", "e\u{301}");
    assert_eq!(
        UnicodeSegmentation::graphemes("é", true).count(),
        UnicodeSegmentation::graphemes("e\u{301}", true).count()
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the graphemes example through its entry point.
    #[test]
    fn test_graphemes_example_entry_point() {
        main();
    }
}
