//! Using items from an external crate (a dependency).

// ANCHOR: example
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

fn main() -> Result<()> {
    a_func()?;
    let value = fallible(false)?;
    println!("value = {value}");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main().expect("main should succeed");
    }

    #[test]
    fn test_error_path() {
        let err = fallible(true).expect_err("should fail");
        assert_eq!(err.to_string(), "something went wrong");
    }
}
