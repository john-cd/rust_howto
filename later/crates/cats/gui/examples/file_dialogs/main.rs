//! Runs the file dialog example.
//!
//! TODO Reviewed ??-??-2026.

mod rfd;

/// Runs the file dialogs example.
fn main() -> anyhow::Result<()> {
    rfd::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the dispatcher through its entry point.
    #[test]
    #[ignore = "needs a desktop session that can show a native file dialog"]
    fn test_main_entry_point() {
        assert!(main().is_ok());
    }
}
