// ANCHOR: example
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

fn main() {
    // Create a new matcher
    let matcher = SkimMatcherV2::default();

    // Define some example strings to search through
    let items = [
        "fuzzy", "hazy", "buzzy", "scuzzy", "cloudy", "fuzzily", "fuzzed",
    ];

    // The pattern to search for
    let pattern = "fuzzy";

    println!("Searching for pattern: '{}'", pattern);

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
    matches.sort_by(|a, b| b.0.cmp(&a.0));

    // Display results
    println!("\nResults (sorted by match score):");
    for (score, item) in matches.clone() {
        println!("Score: {}, Item: '{}'", score, item);
    }

    // Example of highlighting matches
    println!("\nHighlighted matches:");
    for (_, item) in matches {
        // fuzzy match item with pattern, and return the score & matched indices
        // of characters.
        if let Some((score, indices)) = matcher.fuzzy_indices(item, pattern) {
            let mut highlighted = String::new();
            let mut last_idx = 0;

            for &idx in indices.iter() {
                // Add text before the match
                highlighted.push_str(&item[last_idx..idx]);
                // Add the matched character with formatting
                highlighted.push_str("[");
                highlighted.push(item.chars().nth(idx).unwrap());
                highlighted.push_str("]");
                last_idx = idx + 1;
            }

            // Add remaining text
            highlighted.push_str(&item[last_idx..]);
            println!("Score: {}, Highlighted: '{}'", score, highlighted);
        }
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
