//! Runs the drone example.
//!
//! TODO Reviewed ??-??-2026.

/// Sending a flight plan to a Tello drone.
mod drones1;

/// Runs the drones example.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    drones1::run().map_err(Into::into)
}
