// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Diffing lines and words with `similar` and styling the result with
//! `console`.
//!
//! A diff lists the lines (or words) that were added or removed between two
//! texts, the way `git diff` does. It is used for code review, merge tools and
//! test failure messages.

use console::Style;
use similar::ChangeTag;
use similar::TextDiff;

/// Prints line changes and a unified diff with context.
fn line_diff() {
    let old_text = "Hello World\nThis is a test\nfor the `similar` crate\nto demonstrate diffing";
    let new_text = "Hello Rust\nThis is a test\nusing the `similar` crate\nto demonstrate diffing\nwith added line";

    let diff = TextDiff::from_lines(old_text, new_text);
    let add_style = Style::new().green();
    let remove_style = Style::new().red();
    let unchanged_style = Style::new();

    println!("Line changes:");
    for change in diff.iter_all_changes() {
        let (sign, style) = match change.tag() {
            ChangeTag::Delete => ("-", &remove_style),
            ChangeTag::Insert => ("+", &add_style),
            ChangeTag::Equal => (" ", &unchanged_style),
        };
        print!("{}", style.apply_to(format!("{sign}{}", change.value())));
    }

    let mut unified_diff = diff.unified_diff();
    let unified = unified_diff
        .context_radius(2)
        .header("old_file", "new_file");
    println!("\nUnified diff:");
    print!("{unified}");
}

/// Highlights deleted and inserted words in two short sentences.
fn word_diff() {
    let old_text = "The quick brown fox jumps over the lazy dog";
    let new_text = "The quick red fox jumps over the sleepy dog";
    let diff = TextDiff::from_words(old_text, new_text);
    let add_style = Style::new().green();
    let remove_style = Style::new().red();

    print!("Word changes: ");
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => print!(
                "{}",
                remove_style.apply_to(format!("[-{}-]", change.value()))
            ),
            ChangeTag::Insert => print!(
                "{}",
                add_style.apply_to(format!("{{+{}+}}", change.value()))
            ),
            ChangeTag::Equal => print!("{}", change.value()),
        }
    }
    println!();
}

/// Prints line-level and word-level comparisons.
fn main() {
    line_diff();
    word_diff();
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the diff example runs.
    /// Runs the similar example through its entry point.
    #[test]
    fn test_similar_example_entry_point() {
        main();
    }

    /// Line changes are tagged as deletions, insertions and equal lines.
    #[test]
    fn line_changes_are_tagged() {
        let diff = TextDiff::from_lines("a\nb\n", "a\nc\n");
        let tags: Vec<_> = diff
            .iter_all_changes()
            .map(|c| (c.tag(), c.value().trim_end().to_owned()))
            .collect();
        assert_eq!(
            tags,
            [
                (ChangeTag::Equal, "a".to_owned()),
                (ChangeTag::Delete, "b".to_owned()),
                (ChangeTag::Insert, "c".to_owned()),
            ]
        );
    }

    /// The unified diff has the file headers and hunk markers.
    #[test]
    fn unified_diff_has_headers() {
        let diff = TextDiff::from_lines("a\nb\n", "a\nc\n");
        let unified = diff.unified_diff().header("old", "new").to_string();
        assert!(unified.starts_with("--- old\n+++ new\n"), "{unified}");
        assert!(unified.contains("-b\n+c\n"), "{unified}");
    }

    /// Word diffs mark only the changed words.
    #[test]
    fn word_diff_marks_changed_words() {
        let diff = TextDiff::from_words("a quick fox", "a slow fox");
        let changed: Vec<_> = diff
            .iter_all_changes()
            .filter(|c| c.tag() != ChangeTag::Equal)
            .map(|c| (c.tag(), c.value().to_owned()))
            .collect();
        assert_eq!(
            changed,
            [
                (ChangeTag::Delete, "quick".to_owned()),
                (ChangeTag::Insert, "slow".to_owned())
            ]
        );
    }

    /// Identical inputs have no changes.
    #[test]
    fn identical_inputs_have_no_changes() {
        let diff = TextDiff::from_lines("same\n", "same\n");
        assert!(diff.iter_all_changes().all(|c| c.tag() == ChangeTag::Equal));
    }
}
