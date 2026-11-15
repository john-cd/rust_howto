//! Runs the physics engines example.
//!
//! TODO Reviewed ??-??-2026.

/// A ball falling onto the ground in a 2D physics simulation with `rapier2d`.
mod rapier2d;

/// Runs the Rapier 2D example on every supported desktop target.
fn main() {
    rapier2d::run();
}
