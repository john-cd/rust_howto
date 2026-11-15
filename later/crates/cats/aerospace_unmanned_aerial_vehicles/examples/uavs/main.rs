//! Runs the UAV example.
//!
//! TODO Reviewed ??-??-2026.

/// Receiving telemetry from a Tello drone.
mod uavs;

/// Runs the uavs example.
fn main() -> anyhow::Result<()> {
    uavs::run()
}

/// Verifies that the UAV example.
#[cfg(test)]
mod tests {
    use super::*;

    // output-only: the example talks to a real drone.
    #[test]
    #[ignore = "requires real drone"]
    fn test_main_entry_point() -> anyhow::Result<()> {
        main()
    }
}
