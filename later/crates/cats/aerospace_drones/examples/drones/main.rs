//! Runs the drone example.
//!
//! TODO Reviewed ??-??-2026.

mod drones1;

/// Runs the drones example.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    drones1::run().map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    #[ignore = "requires real drone"]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
