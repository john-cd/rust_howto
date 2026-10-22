//! Runs the robot operating systems example.
//!
//! TODO Reviewed ??-??-2026

#[cfg(all(feature = "rosrust", not(windows)))]
mod robotics;

/// Runs the example from the `robotics` module where it is supported.
fn main() {
    #[cfg(all(feature = "rosrust", not(windows)))]
    robotics::run();
}
