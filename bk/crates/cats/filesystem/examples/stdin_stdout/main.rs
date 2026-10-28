//! Runs the standard input and output example.
//!
//! TODO Reviewed ??-??-2026.

mod stdin_stdout;

/// Runs the stdin stdout example.
fn main() -> anyhow::Result<()> {
    stdin_stdout::run()?;
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
