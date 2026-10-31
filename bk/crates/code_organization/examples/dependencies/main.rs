// ANCHOR: example
//! Using items from external crate dependencies.

// Bring commonly used items into scope from the external `anyhow` crate:
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

/// Returns an `anyhow::Result` demonstrating error construction with `anyhow!`.
fn parse_positive_number(text: &str) -> Result<u32> {
    let value: i32 = text
        .trim()
        .parse()
        .map_err(|err| anyhow!("failed to parse integer: {err}"))?;
    if value <= 0 {
        return Err(anyhow!("expected a positive number, got {value}"));
    }
    Ok(value as u32)
}

/// Runs the dependency example and returns `anyhow::Result`.
fn main() -> Result<()> {
    let parsed = parse_positive_number("42")?;
    println!("Parsed value from dependency example: {parsed}");

    let err: Error = anyhow!("sample error struct");
    println!("Created an anyhow::Error: {err}");

    Ok(())
}

// ANCHOR_END: example

/// Runs the dependencies example by calling `main()`.
pub fn run() -> Result<()> {
    // Call through a binding: calling `main` directly trips
    // `clippy::main_recursion`.
    let entry = main;
    entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the dependencies example runs through its entry point.
    #[test]
    fn test_dependencies_example_entry_point() {
        let entry = main;
        assert!(entry().is_ok());
    }

    /// Verifies parsing valid and invalid input.
    #[test]
    fn test_parse_positive_number() {
        assert_eq!(parse_positive_number("10").ok(), Some(10));
        assert!(parse_positive_number("-5").is_err());
        assert!(parse_positive_number("abc").is_err());
    }
}
