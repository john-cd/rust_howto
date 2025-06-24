#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `diff` crate to compare two strings
//! and print the differences.
//!
//! The `diff` crate provides functionality for computing the differences
//! between sequences. In this example, we use it to compare lines within two
//! strings.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! diff = "0.1.13" # Or latest
//! ```
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
            Result::Both(s, _) => println!(" {}", s),
            Result::Left(s) => println!("-{}", s),
            Result::Right(s) => println!("+{}", s),
        }
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
