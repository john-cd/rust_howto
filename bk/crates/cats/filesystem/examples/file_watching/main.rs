//! Runs the file watching example.
//!
//! TODO Reviewed ??-??-2026.

mod notify;

/// Runs the file watching example.
fn main() -> anyhow::Result<()> {
    notify::run()?;
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
