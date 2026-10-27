// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Ranks candidates against a partial query with `nucleo`, the matcher used
//! by the Helix editor.

use nucleo::Config;
use nucleo::Matcher;
use nucleo::pattern::CaseMatching;
use nucleo::pattern::Normalization;
use nucleo::pattern::Pattern;

fn main() -> anyhow::Result<()> {
    let commands = ["save file", "open file", "close window", "open folder"];

    // A `Matcher` owns scratch memory: create it once and reuse it.
    let mut matcher = Matcher::new(Config::DEFAULT);
    // `Smart` case matching is case-insensitive unless the query has capitals.
    let pattern =
        Pattern::parse("opfl", CaseMatching::Smart, Normalization::Smart);

    // Candidates that do not match are dropped; the rest come back sorted by
    // descending score.
    let ranked = pattern.match_list(commands, &mut matcher);
    for (candidate, score) in &ranked {
        println!("{score:>4}  {candidate}");
    }

    anyhow::ensure!(!ranked.is_empty(), "no candidate matched");
    assert_eq!(ranked[0].0, "open file");
    // "save file" has no 'o' before an 'p', so it does not match.
    assert!(ranked.iter().all(|(c, _)| *c != "save file"));
    Ok(())
}
// ANCHOR_END: example

#[allow(clippy::main_recursion)]
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the fuzzy search example through its entry point.
    #[test]
    fn test_fuzzy_search_example_entry_point() -> anyhow::Result<()> {
        main()
    }
}
