#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Splitting a string into grapheme clusters.

use unicode_segmentation::UnicodeSegmentation;

/// Prints the graphemes of a string.
fn main() {
    let name = "José Guimarães\r\n";
    let graphemes =
        UnicodeSegmentation::graphemes(name, true).collect::<Vec<&str>>();
    println!("{graphemes:?}");
    assert_eq!(graphemes[3], "é");
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
