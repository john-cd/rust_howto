//! Runs the clipboard example.
//!
//! TODO Reviewed ??-??-2026.

mod arboard;

/// Runs the clipboard example.
fn main() -> anyhow::Result<()> {
    arboard::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the dispatcher through its entry point.
    #[test]
    #[ignore = "needs a desktop session with a system clipboard"]
    fn test_main_entry_point() {
        assert!(main().is_ok());
    }
}
