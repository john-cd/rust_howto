#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates how to use the `color-eyre` crate for enhanced error reporting.
//!
//! `color_eyre` provides beautiful error reporting and backtraces with colors,
//! making it easier to understand and debug errors.
//! It's especially useful for debugging in command-line applications.
use color_eyre::eyre::Result;
use color_eyre::eyre::eyre;

fn main() -> Result<()> {
    // Initializes color_eyre's error reporting hook. This hook intercepts
    // panics and errors, formatting them with colors and backtraces.
    color_eyre::install()?;

    let result = divide(10, 0)?;
    println!("Result: {}", result);

    Ok(())
}

fn divide(a: i32, b: i32) -> Result<i32> {
    if b == 0 {
        // Use the `eyre!` macro to construct an ad-hoc error from a string.
        Err(eyre!(
            "Attempted to divide {} by zero in the `divide` function.",
            a
        ))
    } else {
        Ok(a / b)
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    assert!(main().is_err());
}
