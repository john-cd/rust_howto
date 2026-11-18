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
        /// Matches a hash followed by an ASCII letter and ASCII word-like characters.
        static ref HASHTAG_REGEX: Regex =
            Regex::new(r"\#[a-zA-Z][0-9a-zA-Z_]*").expect("regular expression should be valid");
    }
    // Matches hashtags that start with a letter and continue with letters, digits, or underscores.
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
    assert!(tags.contains("#dog") && tags.contains("#forever") && tags.contains("#world"));
    assert_eq!(tags.len(), 3);
}
// ANCHOR_END: example

/// Extracts the hashtags of a sample tweet.
pub fn run() {
    main();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the hashtags example through its entry point.
    #[test]
    fn test_hashtags_example_entry_point() {
        main();
    }

    /// Deduplication is case-sensitive and retains the leading hash.
    #[test]
    fn extracts_distinct_ascii_tags() {
        let tags = extract_hashtags("#Rust #rust #Rust #tag_2 #2 #_");
        assert_eq!(tags, HashSet::from(["#Rust", "#rust", "#tag_2"]));
    }

    /// This small pattern allows embedded hashes and partial non-ASCII tags.
    #[test]
    fn documents_extraction_boundaries() {
        assert_eq!(
            extract_hashtags("word#tag #café #éclair"),
            HashSet::from(["#tag", "#caf"])
        );
        assert!(extract_hashtags("no tags here").is_empty());
    }
}
