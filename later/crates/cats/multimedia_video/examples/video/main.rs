//! Runs the video example.
//!
//! TODO Reviewed ??-??-2026.

mod video;

/// Runs the video example.
///
/// # Errors
/// Returns an error if encoder setup or frame encoding fails.
fn main() -> anyhow::Result<()> {
    video::run()
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
