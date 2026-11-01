// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Finds word boundaries with Unicode text segmentation (UAX #29), e.g. to
//! count words or to feed a spell checker.
//!
//! Where a word begins and ends is not obvious: contractions, hyphens, numbers
//! and languages without spaces all complicate it. Unicode Standard Annex #29
//! defines the rules, which double-click selection and word counting follow.

use unicode_segmentation::UnicodeSegmentation;

fn main() -> anyhow::Result<()> {
    let text = "Café déjà-vu: naïve 日本語 can't stop, won't stop.";

    // `unicode_word_indices` yields the byte offset and text of each word,
    // skipping spaces and punctuation but keeping apostrophes within words.
    let words: Vec<(usize, &str)> = text.unicode_word_indices().collect();
    for (offset, word) in &words {
        println!("{offset:>3}  {word}");
    }

    // Offsets are byte offsets, so slicing with them is safe.
    let (offset, word) = words[0];
    assert_eq!(&text[offset..offset + word.len()], "Café");
    assert!(words.iter().any(|(_, w)| *w == "can't"));
    // The hyphen splits "déjà-vu", and each CJK character is its own "word"
    // because UAX #29 has no dictionary for Chinese or Japanese.
    assert_eq!(words.len(), 11);

    // Naive splitting on whitespace disagrees: it keeps punctuation.
    assert!(text.split_whitespace().any(|w| w == "stop,"));
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    // Call through a binding: calling `main` directly trips
    // `clippy::main_recursion`.
    let entry = main;
    entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the word boundaries example through its entry point.
    #[test]
    fn test_word_boundaries_example_entry_point() -> anyhow::Result<()> {
        main()
    }
}
