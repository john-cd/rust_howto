//! Hardware integration example runner.
//!
//! TODO Reviewed ??-??-2026.

mod i2c_driver;

/// Executes all hardware integration example modules.
fn main() {
    i2c_driver::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the driver
    /// module tests its own results).
    #[test]
    fn test_main() {
        main();
    }
}
