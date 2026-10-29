//! Runs the virtualization example.
//!
//! TODO Reviewed ??-??-2026.

mod virtualization1;

/// Runs the virtualization example.
fn main() {
    virtualization1::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    /// Runs the example through its entry point (output-only: the module
    /// tests its own results).
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
