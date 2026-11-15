//! Runs the UAV example.
//!
//! TODO Reviewed ??-??-2026.

/// Receiving telemetry from a Tello drone.
mod uavs;

/// Runs the uavs example.
fn main() -> anyhow::Result<()> {
    uavs::run()
}
