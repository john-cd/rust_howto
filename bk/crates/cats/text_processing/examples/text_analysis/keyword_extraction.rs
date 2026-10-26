// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Extracts key phrases from a paragraph with RAKE (Rapid Automatic Keyword
//! Extraction).

use rake::KeywordScore;
use rake::Rake;
use rake::StopWords;

fn main() -> anyhow::Result<()> {
    // RAKE splits the text at stop words and punctuation. Real applications
    // load a full list for their language, e.g. with `StopWords::from_file`.
    let mut stop_words = StopWords::new();
    for word in [
        "a", "an", "and", "are", "for", "in", "is", "of", "on", "the", "to",
        "with",
    ] {
        stop_words.insert(word.to_owned());
    }
    let rake = Rake::new(stop_words);

    let text = "Compatibility of systems of linear constraints over the set \
                of natural numbers. Criteria of compatibility of a system of \
                linear Diophantine equations are defined.";

    let mut keywords = rake.run(text);
    keywords.sort_by(|a, b| b.score.total_cmp(&a.score));
    for KeywordScore { keyword, score } in keywords.iter().take(3) {
        println!("{score:6.2}  {keyword}");
    }

    // Ties come back in arbitrary order, so compare the top two as a set.
    let mut top: Vec<&str> = keywords
        .iter()
        .take(2)
        .map(|k| k.keyword.as_str())
        .collect();
    top.sort_unstable();
    assert_eq!(
        top,
        ["linear Diophantine equations", "linear constraints over"]
    );
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

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }
}
