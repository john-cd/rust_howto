//! Runs the graceful shutdown examples.
//!
//! TODO Reviewed ??-??-2026.

mod ctrlc_example;
mod tokio_signal;

/// Runs both examples.
fn main() -> anyhow::Result<()> {
    ctrlc_example::run()?;
    tokio_signal::run()?;
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
