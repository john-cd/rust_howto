// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to use the `diff` crate to compare two strings
//! and print the differences.
//!
//! The `diff` crate provides functionality for computing the differences
//! between sequences. In this example, we use it to compare lines within two
//! strings.

use diff::Result;

/// Compares two strings line by line and prints the differences.
fn main() -> anyhow::Result<()> {
    let old_text = "Hello world\nThis is a test\nOf the diff library";
    let new_text =
        "Hello world\nThis is a sample\nOf the diff library\nWith a new line";

    // Computes the diff between the lines of two strings.
    let changeset = diff::lines(old_text, new_text);

    // Print the diff:
    for chunk in changeset {
        match chunk {
            Result::Both(s, _) => println!(" {s}"),
            Result::Left(s) => println!("-{s}"),
            Result::Right(s) => println!("+{s}"),
        }
    }

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the diff example through its entry point.
    #[test]
    fn test_diff_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// Unchanged, removed and added lines are classified.
    #[test]
    fn classifies_lines() {
        let changes = diff::lines("a\nb\nc", "a\nB\nc\nd");
        assert_eq!(
            changes,
            [
                Result::Both("a", "a"),
                Result::Left("b"),
                Result::Right("B"),
                Result::Both("c", "c"),
                Result::Right("d"),
            ]
        );
    }

    /// Identical texts have only unchanged lines; empty texts have none.
    #[test]
    fn identical_and_empty_inputs() {
        assert!(
            diff::lines("x\ny", "x\ny")
                .iter()
                .all(|c| matches!(c, Result::Both(..)))
        );
        assert!(diff::lines("", "").is_empty());
    }
}
