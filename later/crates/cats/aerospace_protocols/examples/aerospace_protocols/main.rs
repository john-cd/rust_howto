//! Runs the MAVLink example.
//!
//! TODO Reviewed ??-??-2026.

/// Encode and decode a MAVLink heartbeat without a vehicle or network.
mod aero_protocols;

/// Runs the aerospace protocols example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    aero_protocols::run()
}

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
