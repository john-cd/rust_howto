//! Runs the multimedia example.
//!
//! TODO Reviewed ??-??-2026.

mod multimedia1;

/// Runs the multimedia example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    multimedia1::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    #[ignore = "requires an audio output device"]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
