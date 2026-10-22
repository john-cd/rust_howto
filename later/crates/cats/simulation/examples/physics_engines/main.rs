//! Runs the physics engines example.
//!
//! TODO Reviewed ??-??-2026

#[cfg(not(windows))]
mod rapier2d;

/// Runs the example from the `rapier2d` module where it is supported.
fn main() {
    #[cfg(not(windows))]
    // TODO - add a Windows-compatible physics engine example
    rapier2d::run();
}
