// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example of a `main` function returning a `Result`,
//! in this case a specialized `Result` type for I/O operations.

use std::fs::File;

/// Example of a `main` function returning a `Result`,
/// in this case a specialized `Result` type for I/O operations.
fn main() -> std::io::Result<()> {
    let _f = File::open("non_existent_file.txt")?; // The ? operator returns early, if there is an error.
    println!("File opened successfully (this won't print).");
    // Would return `Ok(())` on success.
    // The inner parens represent the unit type, i.e. the absence of a
    // meaningful return value.
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> std::io::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the main fn with result example's main path succeeds.
    /// Runs the main fn with result example through its entry point.
    #[test]
    fn test_main_fn_with_result_example_entry_point() {
        assert!(main().is_err());
    }
}
