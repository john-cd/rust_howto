#![allow(dead_code)]
// ANCHOR: example
//! # Text Diffing Example
//!
//! This example demonstrates how to use the `similar` crate to perform text
//! diffing and display the differences in a user-friendly format. It showcases
//! both line-by-line and word-by-word diffing, along with styled output using
//! the `console` crate.
//!
//! ## Dependencies
//!
//! Add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! similar = "2.7.0" # Or latest
//! console = "0.15.11" # Or latest
//! ```
use std::fmt;

use console::Style;
use similar::ChangeTag;
use similar::TextDiff;

struct Line(Option<usize>, String);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    {}", self.1),
            Some(idx) => write!(f, "{:4} {}", idx + 1, self.1),
        }
    }
}

/// Performs a line-by-line diff between two text strings and prints the
/// differences with styling. It also demonstrates the use of the unified
/// diff format for a more compact representation of the changes.
fn line_diff() {
    // Original and modified text:
    let old_text = "Hello World\nThis is a test\nfor the `similar` crate\nto demonstrate diffing";
    let new_text = "Hello Rust\nThis is a test\nusing the `similar` crate\nto demonstrate diffing\nwith added line";

    // Generate a diff with context:
    let diff = TextDiff::from_lines(old_text, new_text);

    // Define styles for different types of changes:
    let add_style = Style::new().green();
    let remove_style = Style::new().red();
    let unchanged_style = Style::new();

    // Print the styled diff:
    println!("Text Difference:");
    for change in diff.iter_all_changes() {
        let (sign, style) = match change.tag() {
            ChangeTag::Delete => ("-", &remove_style),
            ChangeTag::Insert => ("+", &add_style),
            ChangeTag::Equal => (" ", &unchanged_style),
        };

        // Format the line with line number and styling:
        let line = Line(
            change.old_index().or(change.new_index()),
            format!("{}{}", sign, change.value()),
        );
        println!("{}", style.apply_to(line));
    }

    // Use the `unified_diff`` format for comparison:
    println!("\nUnified Diff Format:");
    let mut unified = diff.unified_diff();
    let unified = unified.context_radius(2).header("old_file", "new_file");

    print!("{unified}");
}

/// Performs a word-by-word diff between two text strings and prints the
/// differences with styling. This function splits the text into words and
/// whitespace, highlighting the changes at the word level.
fn word_diff() {
    let text1 = "The quick brown fox jumps over the lazy dog";
    let text2 = "The quick red fox jumps over the sleepy dog";

    let diff = TextDiff::from_words(text1, text2);

    print!("\nWord-by-word changes: ");
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => print!("\x1b[31m{}\x1b[0m", change.value()),
            ChangeTag::Insert => print!("\x1b[32m{}\x1b[0m", change.value()),
            ChangeTag::Equal => print!("{}", change.value()),
        }
    }
    println!();
}

fn main() {
    line_diff();
    word_diff();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
