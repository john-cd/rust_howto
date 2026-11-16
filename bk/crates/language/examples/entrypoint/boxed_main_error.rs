//! Shows a main function that can return different error types.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use std::error::Error;

/// Runs the command and propagates any error.
fn main() -> Result<(), Box<dyn Error>> {
    let number: u32 = "42".parse()?;
    println!("{number}");
    Ok(())
}
// ANCHOR_END: example

/// A main function with a boxed error type can return success.
#[test]
fn main_returns_success() -> Result<(), Box<dyn Error>> {
    main()
}
