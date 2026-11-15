//! Runs the ahead-of-time macro helper example.
//!
//! TODO Reviewed ??-??-2026.

/// Parses a function and renames it using `proc_macro2`, `syn`, and `quote`.
mod watt;

/// Runs the `watt` token-transformation example.
fn main() -> anyhow::Result<()> {
    watt::run()
}

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
