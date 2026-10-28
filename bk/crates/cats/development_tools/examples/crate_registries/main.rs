//! Crate registries examples.
//!
//! TODO Reviewed ??-??-2026.

mod crates_io;

/// Runs the crate registries example.
fn main() -> anyhow::Result<()> {
    crates_io::run()?;
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
