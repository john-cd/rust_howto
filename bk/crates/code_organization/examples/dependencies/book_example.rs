// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Using items from an external crate (a dependency).

// Bring the following items into scope:
use anyhow::Error; // A struct.
use anyhow::Result; // A type alias.
use anyhow::anyhow; // The `anyhow!` macro.

/// Refers to an item of `anyhow` by its full path, without a `use`.
fn a_func() -> anyhow::Result<()> {
    Ok(())
}

/// Refers to `Result` without having to write its full path every time.
fn fallible(fail: bool) -> Result<u32> {
    if fail {
        // Build an `Error` with the `anyhow!` macro.
        let e: Error = anyhow!("something went wrong");
        return Err(e);
    }
    Ok(1)
}

/// Program entry point for the external dependency example.
///
/// # Errors
///
/// Returns an error if the example returns one.
fn main() -> Result<()> {
    example()
}

/// Uses imported names from `anyhow` and prints the successful result.
///
/// # Errors
///
/// Returns an error if `a_func` or `fallible(false)` returns one.
fn example() -> Result<()> {
    a_func()?;
    let value = fallible(false)?;
    println!("value = {value}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the book example.
pub fn run() -> Result<()> {
    let entry_point = main;
    entry_point()
}
/// Using items from an external crate (a dependency).
#[cfg(test)]
mod tests {
    use super::*;

    /// Returns the expected error message for a failing dependency.
    #[test]
    fn test_error_path() {
        let err = fallible(true).expect_err("should fail");
        assert_eq!(err.to_string(), "something went wrong");
    }
}

#[cfg(test)]
mod private_main_tests {
    use super::main;

    #[test]
    fn private_main_succeeds() {
        let entry_point = main;
        entry_point().expect("the example entry point should succeed");
    }
}
