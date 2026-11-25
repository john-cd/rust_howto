// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Redacts many words in a log line with one reusable, case-insensitive
//! `aho-corasick` automaton.
//!
//! This example masks configured fruit names in sample text. Build the
//! Aho-Corasick matcher once and reuse it for each input.

use aho_corasick::AhoCorasick;
use aho_corasick::MatchKind;

/// Builds the automaton once. Building is the expensive part.
fn redactor(words: &[&str]) -> Result<AhoCorasick, aho_corasick::BuildError> {
    AhoCorasick::builder()
        .ascii_case_insensitive(true)
        // Prefer the longest word at a given position:
        // "pineapple" over "pine".
        .match_kind(MatchKind::LeftmostLongest)
        .build(words)
}

/// Replaces every match with `***`.
fn redact(ac: &AhoCorasick, text: &str) -> String {
    let replacements = vec!["***"; ac.patterns_len()];
    ac.replace_all(text, &replacements)
}

/// Finds and redacts configured fruit names in a sample string.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
fn main() -> anyhow::Result<()> {
    let ac = redactor(&["pine", "pineapple", "pear"])?;

    let line = "Basket: Pineapple, PEAR, pine";
    let redacted = redact(&ac, line);
    println!("{redacted}");

    assert_eq!(redacted, "Basket: ***, ***, ***");
    // The same automaton can be reused for any number of texts.
    assert_eq!(redact(&ac, "no secrets"), "no secrets");
    Ok(())
}
// ANCHOR_END: example

/// Finds and redacts configured fruit names in a sample string.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> anyhow::Result<()> {
    // Call through a binding: calling `main` directly trips
    // `clippy::main_recursion`.
    let entry = main;
    entry()
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the aho corasick redact example through its entry point.
    #[test]
    fn test_aho_corasick_redact_example_entry_point() -> anyhow::Result<()> {
        main()
    }
}
