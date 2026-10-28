//! Runs the crash-safe atomic file replacement example.
//!
//! TODO Reviewed ??-??-2026.

mod atomic_write;

/// Runs the atomic write example.
fn main() -> anyhow::Result<()> {
    atomic_write::run()?;
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
