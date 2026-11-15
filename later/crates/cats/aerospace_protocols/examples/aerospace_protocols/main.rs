//! Runs the MAVLink example.
//!
//! TODO Reviewed ??-??-2026.

/// Encode and decode a MAVLink heartbeat without a vehicle or network.
mod aero_protocols;

/// Runs the aerospace protocols example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    aero_protocols::run()
}
