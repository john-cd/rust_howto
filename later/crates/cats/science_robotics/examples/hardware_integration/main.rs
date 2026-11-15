//! Hardware integration example runner.
//!
//! TODO Reviewed ??-??-2026.

/// Generic I2C driver example using `embedded-hal` 1.0 traits.
mod i2c_driver;

/// Executes all hardware integration example modules.
fn main() {
    i2c_driver::run();
}

/// module tests its own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// module tests its own results).
    #[test]
    fn test_main() {
        main();
    }
}
