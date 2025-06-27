#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates the use of the `anyhow` crate for error handling.

use anyhow::Context;
use anyhow::Result;

/// Simulates a function that returns an error.
fn do_something() -> Result<()> {
    Err(anyhow::Error::msg("Some Error"))
}

fn main() -> anyhow::Result<()> {
    // Provide context to the error:
    do_something().context("Failed to do the important thing")?;

    // Demonstrates adding context to a standard library error.
    // The closure is only evaluated if there is an error.
    let _content = std::fs::read("/notafile.txt")
        .with_context(|| "Failed to read instrs from file".to_string())?;

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() {
    let res = main();
    println!("{res:?}");
    assert!(res.is_err())
}
