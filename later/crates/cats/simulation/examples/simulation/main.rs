//! Runs the simulation example.
//!
//! TODO Reviewed ??-??-2026

#[cfg(not(windows))] // TODO review Windows support for simulation examples
mod simulation1;

/// Runs the example from the `simulation1` module where it is supported.
fn main() {
    #[cfg(not(windows))]
    simulation1::run();
}
