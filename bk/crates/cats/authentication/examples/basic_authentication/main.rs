//! Runs the basic authentication example.
//!
//! TODO Reviewed ??-??-2026.

mod basic;

/// Runs the basic authentication example.
fn main() -> anyhow::Result<()> {
    basic::run()?;
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
