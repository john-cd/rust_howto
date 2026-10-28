//! Runs the symbolic links example.
//!
//! TODO Reviewed ??-??-2026.

mod symbolic_links;

/// Runs the symbolic links example.
fn main() -> anyhow::Result<()> {
    symbolic_links::run()?;
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
