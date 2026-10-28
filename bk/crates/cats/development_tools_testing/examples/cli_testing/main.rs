//! Examples of testing command-line programs and reading coverage reports.
//!
//! TODO Reviewed ??-??-2026.

mod lcov_summary;

/// Runs the examples.
fn main() -> anyhow::Result<()> {
    lcov_summary::run()?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
