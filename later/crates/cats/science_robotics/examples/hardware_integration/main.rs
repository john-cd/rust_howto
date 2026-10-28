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

    #[test]
    fn test_main() {
        main();
    }
}
