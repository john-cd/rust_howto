// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Redacts many words in a log line with one reusable, case-insensitive
//! `aho-corasick` automaton.

use aho_corasick::AhoCorasick;
use aho_corasick::MatchKind;

/// Builds the automaton once. Building is the expensive part.
fn redactor(words: &[&str]) -> Result<AhoCorasick, aho_corasick::BuildError> {
    AhoCorasick::builder()
        .ascii_case_insensitive(true)
        // Prefer the longest word at a given position:
        // "password" over "pass".
        .match_kind(MatchKind::LeftmostLongest)
        .build(words)
}

/// Replaces every match with `***`.
fn redact(ac: &AhoCorasick, text: &str) -> String {
    let replacements = vec!["***"; ac.patterns_len()];
    ac.replace_all(text, &replacements)
}

fn main() -> anyhow::Result<()> {
    let ac = redactor(&["pass", "password", "token"])?;

    let line = "User sent Password=hunter2 and TOKEN=abc, pass it on";
    let redacted = redact(&ac, line);
    println!("{redacted}");

    assert_eq!(redacted, "User sent ***=hunter2 and ***=abc, *** it on");
    // The same automaton can be reused for any number of texts.
    assert_eq!(redact(&ac, "no secrets"), "no secrets");
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    // Call through a binding: calling `main` directly trips `clippy::main_recursion`.
    let entry = main;
    entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the aho corasick redact example through its entry point.
    #[test]
    fn test_aho_corasick_redact_example_entry_point() -> anyhow::Result<()> {
        main()
    }
}
