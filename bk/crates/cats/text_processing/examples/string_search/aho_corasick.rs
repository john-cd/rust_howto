#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Searching a text for several patterns at once with the Aho-Corasick
//! algorithm.

use std::collections::HashMap;

use aho_corasick::AhoCorasick;

/// Searches a text for several patterns at once.
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
            "Pattern '{}' at byte range {start}..{end}",
            patterns[pattern_idx]
        );
    }

    // Using with replacement:
    let ac = AhoCorasick::new(patterns)?;
    let result = ac.replace_all(text, &["APPLE", "BANANA", "ORANGE"]);
    println!("\nReplaced text: {result}");

    // Choose leftmost-first semantics when patterns overlap:
    let patterns = &["abc", "bc", "c"];

    // Use leftmost-first match semantics, which reports leftmost matches, like
    // a Perl regex. When there are multiple possible leftmost matches, the
    // match corresponding to the pattern that appeared earlier when
    // constructing the automaton is reported.
    let ac = AhoCorasick::builder()
        .match_kind(aho_corasick::MatchKind::LeftmostFirst)
        .build(patterns)?;

    let text = "abcdef";
    println!("\nLeftmost-first matches in 'abcdef':");
    for mat in ac.find_iter(text) {
        println!(
            "Pattern '{}' at byte range {}..{}",
            patterns[mat.pattern()],
            mat.start(),
            mat.end()
        );
    }

    // Using with byte offsets:
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

    // Example: counting occurrences:
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

    // Build one automaton for a larger vocabulary. ASCII case-insensitive
    // matching avoids lowercasing the input and reports original byte spans.
    let keywords: Vec<String> = (0..100)
        .map(|index| format!("keyword-{index:03}"))
        .collect();
    let keyword_refs: Vec<&str> = keywords.iter().map(String::as_str).collect();
    let automaton = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .build(&keyword_refs)?;
    let text = "KEYWORD-001 and keyword-099; keyword-001 appears twice";
    let matches: Vec<_> = automaton
        .find_iter(text)
        .map(|mat| (mat.pattern().as_usize(), mat.start(), mat.end()))
        .collect();
    println!("\nMatches from a 100-keyword vocabulary:");
    for (pattern, start, end) in &matches {
        println!("{} at byte range {start}..{end}", keywords[*pattern]);
    }

    // Compare with checking every lowercased keyword using `contains`.
    let folded_text = text.to_ascii_lowercase();
    let contains_matches: Vec<bool> = keywords
        .iter()
        .map(|keyword| folded_text.contains(&keyword.to_ascii_lowercase()))
        .collect();
    let mut aho_matches = vec![false; keywords.len()];
    for (pattern, _, _) in &matches {
        aho_matches[*pattern] = true;
    }
    assert_eq!(aho_matches, contains_matches);
    assert_eq!(matches.len(), 3);

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the aho corasick example through its entry point.
    #[test]
    fn test_aho_corasick_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
