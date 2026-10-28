#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Searching a text for several patterns at once with the Aho-Corasick
//! algorithm.

use std::collections::HashMap;

use aho_corasick::AhoCorasick;

/// Finds basic pattern matches in text using Aho-Corasick.
fn find_basic_matches(
    patterns: &[&str],
    text: &str,
) -> anyhow::Result<Vec<(usize, usize, usize)>> {
    let ac = AhoCorasick::new(patterns)?;
    let mut matches = vec![];
    for mat in ac.find_iter(text) {
        matches.push((mat.pattern().as_usize(), mat.start(), mat.end()));
    }
    Ok(matches)
}

/// Replaces matched patterns in text with corresponding replacements.
fn replace_patterns(
    patterns: &[&str],
    replacements: &[&str],
    text: &str,
) -> anyhow::Result<String> {
    let ac = AhoCorasick::new(patterns)?;
    let result = ac.replace_all(text, replacements);
    Ok(result)
}

/// Finds matches with leftmost-first match semantics.
fn find_leftmost_matches(
    patterns: &[&str],
    text: &str,
) -> anyhow::Result<Vec<(usize, usize, usize)>> {
    let ac = AhoCorasick::builder()
        .match_kind(aho_corasick::MatchKind::LeftmostFirst)
        .build(patterns)?;
    let mut matches = vec![];
    for mat in ac.find_iter(text) {
        matches.push((mat.pattern().as_usize(), mat.start(), mat.end()));
    }
    Ok(matches)
}

/// Counts occurrences of each pattern found in the text.
fn count_occurrences<'a>(
    patterns: &'a [&'a str],
    text: &'a str,
) -> anyhow::Result<HashMap<&'a str, usize>> {
    let ac = AhoCorasick::new(patterns)?;
    let mut counts = HashMap::new();
    for mat in ac.find_iter(text) {
        let pattern = patterns[mat.pattern().as_usize()];
        *counts.entry(pattern).or_insert(0) += 1;
    }
    Ok(counts)
}

/// Searches a text against a generated vocabulary with ASCII case-insensitivity.
fn search_vocabulary(text: &str) -> anyhow::Result<Vec<(usize, usize, usize)>> {
    let keywords: Vec<String> = (0..100)
        .map(|index| format!("keyword-{index:03}"))
        .collect();
    let keyword_refs: Vec<&str> = keywords.iter().map(String::as_str).collect();
    let automaton = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .build(&keyword_refs)?;
    let matches: Vec<_> = automaton
        .find_iter(text)
        .map(|mat| (mat.pattern().as_usize(), mat.start(), mat.end()))
        .collect();
    Ok(matches)
}

/// Searches a text for several patterns at once.
fn main() -> anyhow::Result<()> {
    let patterns = &["apple", "banana", "orange"];
    let text = "I like apple and banana smoothies";

    let matches = find_basic_matches(patterns, text)?;
    println!("Basic matches:");
    for (pattern_idx, start, end) in &matches {
        println!(
            "Pattern '{}' at byte range {start}..{end}",
            patterns[*pattern_idx]
        );
    }

    let replaced = replace_patterns(patterns, &["APPLE", "BANANA", "ORANGE"], text)?;
    println!("\nReplaced text: {replaced}");

    let lm_patterns = &["abc", "bc", "c"];
    let lm_matches = find_leftmost_matches(lm_patterns, "abcdef")?;
    println!("\nLeftmost-first matches in 'abcdef':");
    for (pattern_idx, start, end) in &lm_matches {
        println!(
            "Pattern '{}' at byte range {start}..{end}",
            lm_patterns[*pattern_idx]
        );
    }

    let count_patterns = &["he", "she", "his", "hers"];
    let sample = "he said that she was using his book, not hers";
    let counts = count_occurrences(count_patterns, sample)?;
    println!("\nWord counts:");
    for (pattern, count) in counts {
        println!("{pattern}: {count}");
    }

    let vocab_text = "KEYWORD-001 and keyword-099; keyword-001 appears twice";
    let vocab_matches = search_vocabulary(vocab_text)?;
    assert_eq!(vocab_matches.len(), 3);

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

    #[test]
    fn test_find_basic_matches() -> anyhow::Result<()> {
        let patterns = &["apple", "banana"];
        let matches = find_basic_matches(patterns, "apple pie")?;
        assert_eq!(matches, vec![(0, 0, 5)]);

        let empty_matches = find_basic_matches(patterns, "no fruit here")?;
        assert!(empty_matches.is_empty());
        Ok(())
    }

    #[test]
    fn test_replace_patterns() -> anyhow::Result<()> {
        let patterns = &["foo", "bar"];
        let res = replace_patterns(patterns, &["FOO", "BAR"], "foo and bar")?;
        assert_eq!(res, "FOO and BAR");
        Ok(())
    }

    #[test]
    fn test_leftmost_matches() -> anyhow::Result<()> {
        let patterns = &["abc", "bc"];
        let matches = find_leftmost_matches(patterns, "abc")?;
        assert_eq!(matches, vec![(0, 0, 3)]);
        Ok(())
    }

    #[test]
    fn test_count_occurrences() -> anyhow::Result<()> {
        let patterns = &["cat", "dog"];
        let counts = count_occurrences(patterns, "cat and cat and dog")?;
        assert_eq!(counts.get("cat"), Some(&2));
        assert_eq!(counts.get("dog"), Some(&1));
        Ok(())
    }
}
