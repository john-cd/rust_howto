#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Fuzzy matching of strings with `fuzzy-matcher`.

use fuzzy_matcher::FuzzyMatcher as _;
use fuzzy_matcher::skim::SkimMatcherV2;

/// Scores and ranks strings against a pattern.
fn main() {
    // Create a new matcher
    let matcher = SkimMatcherV2::default();

    // Define some example strings to search through
    let items = [
        "fuzzy", "hazy", "buzzy", "scuzzy", "cloudy", "fuzzily", "fuzzed",
        "fuzzé",
    ];

    // The pattern to search for
    let pattern = "fuz";

    println!("Searching for pattern: '{pattern}'");

    // Find all matches and their scores
    let mut matches: Vec<(i64, &str)> = items
        .iter()
        .filter_map(|item| {
            matcher
                .fuzzy_match(item, pattern)
                .map(|score| (score, *item))
        })
        .collect();

    // Sort by score (highest first)
    matches.sort_by_key(|entry| std::cmp::Reverse(entry.0));

    // Display results
    println!("\nResults (sorted by match score):");
    for (score, item) in &matches {
        println!("Score: {score}, Item: '{item}'");
    }

    // Example of highlighting matches
    println!("\nHighlighted matches:");
    for (_, item) in matches {
        // fuzzy match item with pattern, and return the score & matched indices
        // of characters.
        if let Some((score, indices)) = matcher.fuzzy_indices(item, pattern) {
            let mut highlighted = String::new();
            for (char_idx, ch) in item.chars().enumerate() {
                if indices.contains(&char_idx) {
                    highlighted.push('[');
                    highlighted.push(ch);
                    highlighted.push(']');
                } else {
                    highlighted.push(ch);
                }
            }
            println!("Score: {score}, Highlighted: '{highlighted}'");
        }
    }
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
