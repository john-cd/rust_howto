//! Runs the artificial intelligence examples.
//!
//! TODO Reviewed ??-??-2026.

mod astar;
mod bonsai_bt;

/// Runs the artificial intelligence examples.
fn main() -> anyhow::Result<()> {
    astar::run()?;
    bonsai_bt::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
