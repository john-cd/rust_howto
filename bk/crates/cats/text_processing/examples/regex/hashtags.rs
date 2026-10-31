// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Extracting hashtags from a text with a regular expression.
//!
//! A regular expression (regex) describes a text pattern. Capture groups pick
//! out parts of a match; here the word after the `#`. Compile a regex once and
//! reuse it, because compilation is costly.

use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

/// Returns the distinct hashtags in `text`.
fn extract_hashtags(text: &str) -> HashSet<&str> {
    lazy_static! {
    /// Matches hashtags that start with a letter and continue with letters, digits, or underscores.
            static ref HASHTAG_REGEX: Regex =
                Regex::new(r"\#[a-zA-Z][0-9a-zA-Z_]*").expect("regular expression should be valid");
        }
    HASHTAG_REGEX
        .find_iter(text)
        .map(|mat| mat.as_str())
        .collect()
}

/// Extracts the hashtags of a sample tweet.
fn main() {
    let tweet = "Hey #world, I just got my new #dog, say hello to Till. #dog #forever #2 #_ ";
    let tags = extract_hashtags(tweet);
    println!("{tags:?}");
    assert!(
        tags.contains("#dog")
            && tags.contains("#forever")
            && tags.contains("#world")
    );
    assert_eq!(tags.len(), 3);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the hashtags example through its entry point.
    #[test]
    fn test_hashtags_example_entry_point() {
        main();
    }
}
