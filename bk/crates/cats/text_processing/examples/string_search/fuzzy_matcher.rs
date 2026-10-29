// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Fuzzy matching of strings with `fuzzy-matcher`.

use fuzzy_matcher::FuzzyMatcher as _;
use fuzzy_matcher::skim::SkimMatcherV2;

/// Scores and ranks strings against a pattern.
fn main() {
    // Create a matcher that scores similar strings.
    let matcher = SkimMatcherV2::default();

    // Define candidate strings for the search.
    let items = [
        "fuzzy", "hazy", "buzzy", "scuzzy", "cloudy", "fuzzily", "fuzzed",
        "fuzzé",
    ];

    // Search for strings similar to this pattern.
    let pattern = "fuz";

    println!("Searching for pattern: '{pattern}'");

    // Collect each matching string with its score.
    let mut matches: Vec<(i64, &str)> = items
        .iter()
        .filter_map(|item| {
            matcher
                .fuzzy_match(item, pattern)
                .map(|score| (score, *item))
        })
        .collect();

    // Sort the matches from highest score to lowest.
    matches.sort_by_key(|entry| std::cmp::Reverse(entry.0));

    // Display the ranked results.
    println!("\nResults (sorted by match score):");
    for (score, item) in &matches {
        println!("Score: {score}, Item: '{item}'");
    }

    // Show each match with its matched characters highlighted.
    println!("\nHighlighted matches:");
    for (_, item) in matches {
        // Get the match score and the indices of matched characters.
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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the fuzzy matcher example through its entry point.
    #[test]
    fn test_fuzzy_matcher_example_entry_point() {
        main();
    }

    /// Only candidates containing the pattern characters in order match.
    #[test]
    fn matches_subsequences_only() {
        let matcher = SkimMatcherV2::default();
        for item in ["fuzzy", "fuzzily", "fuzzed", "fuzzé"] {
            assert!(matcher.fuzzy_match(item, "fuz").is_some(), "{item}");
        }
        for item in ["hazy", "buzzy", "cloudy"] {
            assert!(matcher.fuzzy_match(item, "fuz").is_none(), "{item}");
        }
    }

    /// The matched character indices are returned.
    #[test]
    fn indices_point_at_matched_characters() {
        let matcher = SkimMatcherV2::default();
        let (_, indices) = matcher.fuzzy_indices("fuzzy", "fuz").unwrap();
        assert_eq!(indices, [0, 1, 2]);
    }

    /// An exact match scores at least as well as a partial one.
    #[test]
    fn exact_match_scores_higher() {
        let matcher = SkimMatcherV2::default();
        let exact = matcher.fuzzy_match("fuz", "fuz").unwrap();
        let partial = matcher.fuzzy_match("a_f_u_z", "fuz").unwrap();
        assert!(exact > partial);
    }
}
