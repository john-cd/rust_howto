//! Example building cargo plugins and executing shells
//!
//! TODO Reviewed ??-??-2026.

mod xshell;

/// Runs the building example.
fn main() -> anyhow::Result<()> {
    xshell::run()?;
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
