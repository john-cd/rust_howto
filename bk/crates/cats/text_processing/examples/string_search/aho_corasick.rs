#![allow(dead_code)]
// ANCHOR: example
use std::collections::HashMap;

use aho_corasick::AhoCorasick;

fn main() -> anyhow::Result<()> {
    // Search for occurrences of multiple patterns simultaneously.
    let patterns = &["apple", "banana", "orange"];
    let ac = AhoCorasick::new(patterns)?;

    // For case-insensitive search, use:
    // let ac = AhoCorasick::builder()
    // .ascii_case_insensitive(true)
    // .build(patterns)?;

    let text = "I like apple and banana smoothies";
    let mut matches = vec![];

    for mat in ac.find_iter(text) {
        matches.push((mat.pattern(), mat.start(), mat.end()));
    }

    println!("Basic matches:");
    for (pattern_idx, start, end) in matches {
        println!(
            "Pattern '{}' at byte range {}..{}",
            patterns[pattern_idx], start, end
        );
    }

    // Using with replacement
    let ac = AhoCorasick::new(patterns)?;
    let result = ac.replace_all(text, &["APPLE", "BANANA", "ORANGE"]);
    println!("\nReplaced text: {result}");

    // Finding all matches with overlapping patterns
    let patterns = &["abc", "bc", "c"];

    // Use leftmost-first match semantics, which reports leftmost matches, like
    // a Perl regex. When there are multiple possible leftmost matches, the
    // match corresponding to the pattern that appeared earlier when
    // constructing the automaton is reported.
    let ac = AhoCorasick::builder()
        .match_kind(aho_corasick::MatchKind::LeftmostFirst)
        .build(patterns)?;

    let text = "abcdef";
    println!("\nOverlapping matches in 'abcdef':");
    for mat in ac.find_iter(text) {
        println!(
            "Pattern '{}' at byte range {}..{}",
            patterns[mat.pattern()],
            mat.start(),
            mat.end()
        );
    }

    // Using with byte offsets
    let patterns = &[b"abc", b"def"];
    let ac = AhoCorasick::new(patterns)?;

    let text = b"abcdef";
    println!("\nMatches in byte sequence:");
    for mat in ac.find_iter(text) {
        println!(
            "{:?} at byte range {}..{}",
            mat.pattern(),
            mat.start(),
            mat.end()
        );
    }

    // Example: counting occurrences
    let patterns = &["he", "she", "his", "hers"];
    let ac = AhoCorasick::new(patterns)?;

    let text = "he said that she was using his book, not hers";
    let mut counts = HashMap::new();

    for mat in ac.find_iter(text) {
        let pattern = patterns[mat.pattern()];
        *counts.entry(pattern).or_insert(0) += 1;
    }

    println!("\nWord counts:");
    for (pattern, count) in counts {
        println!("{pattern}: {count}");
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
