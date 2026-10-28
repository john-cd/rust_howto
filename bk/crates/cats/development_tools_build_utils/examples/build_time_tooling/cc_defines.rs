// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calls C functions that return preprocessor definitions supplied by `cc`.

/// Runs the C preprocessor definitions example.
///
/// # Errors
///
/// Returns an error if reading C definitions fails.
fn main() -> anyhow::Result<()> {
    super::cc_defines1::run()
}

// ANCHOR_END: example

/// Runs the C preprocessor definitions example.
///
/// # Errors
///
/// Returns an error if reading C definitions fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the C preprocessor definitions example runs successfully.
    ///
    /// # Errors
    ///
    /// Returns an error if reading C definitions fails.
    #[test]
    fn test_main_entry_point() -> anyhow::Result<()> {
        main()
    }
}
