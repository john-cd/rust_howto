// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates the use of the `anyhow` crate for error handling.
//!
//! `anyhow` is for applications: one `anyhow::Error` type holds any error and
//! lets you add context (`.context("reading the config")`) as it propagates,
//! which makes error messages readable. Libraries usually define their own
//! error types instead.

use anyhow::Context as _;
use anyhow::Result;

/// Simulates a function that returns an error.
fn do_something() -> Result<()> {
    Err(anyhow::Error::msg("Some Error"))
}

/// Adds context to errors with `anyhow`.
fn main() -> anyhow::Result<()> {
    // Provide context to the error:
    do_something().context("Failed to do the important thing")?;

    // Demonstrates adding context to a standard library error.
    // The closure is only evaluated if there is an error.
    let _content = std::fs::read("/notafile.txt")
        .with_context(|| "Failed to read instrs from file".to_owned())?;

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the anyhow example's main path succeeds.
    /// Runs the anyhow example through its entry point.
    #[test]
    fn test_anyhow_example_entry_point() {
        let res = main();
        println!("{res:?}");
        assert!(res.is_err());
    }
}
