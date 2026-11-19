//! Runs the Neon example.
//!
//! TODO Reviewed ??-??-2026.

/// This example demonstrates how to create a simple Neon module that exports a function to
/// JavaScript.
mod neon;

/// Runs the Neon example's host-independent path.
fn main() -> anyhow::Result<()> {
    neon::run()
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn test_main_entry_point() {
        main().expect("the Neon example should run successfully");
    }
}
