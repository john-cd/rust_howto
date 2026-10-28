//! Runs the temporary file example.
//!
//! TODO Reviewed ??-??-2026.

mod tempfile;

/// Runs the tempfile example.
fn main() -> anyhow::Result<()> {
    tempfile::run()?;
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
