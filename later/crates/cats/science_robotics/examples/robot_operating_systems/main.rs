//! Runs the robot operating systems example.
//!
//! TODO Reviewed ??-??-2026.

// This example remains as source but is disabled in Cargo.toml until ROS 1's
// affected hyper, idna, time, and traitobject dependency chain can be updated.

#[cfg(all(feature = "rosrust", not(windows)))]
mod robotics;

/// Runs the example from the `robotics` module where it is supported.
fn main() {
    #[cfg(all(feature = "rosrust", not(windows)))]
    robotics::run();
}
