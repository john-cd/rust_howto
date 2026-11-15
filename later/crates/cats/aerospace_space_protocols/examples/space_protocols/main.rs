//! Runs the space protocols example.
//!
//! TODO Reviewed ??-??-2026.

/// Create and parse a PUS-C telecommand.
mod pus_telecommand;

/// Create and read a PUS-C connection-test telemetry packet.
mod pus_telemetry;

/// Build and parse CCSDS space packets and ECSS PUS packets.
mod space_protocols;

/// Runs the space protocols example.
fn main() {
    space_protocols::run();
}
