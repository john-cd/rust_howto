//! Runs the file metadata example.
//!
//! TODO Reviewed ??-??-2026.

mod infer;
mod metadata;

/// Runs the metadata example.
fn main() -> anyhow::Result<()> {
    metadata::run()?;
    infer::run()?;
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
