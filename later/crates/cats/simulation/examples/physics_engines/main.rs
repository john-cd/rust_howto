//! Runs the physics engines example.
//!
//! TODO Reviewed ??-??-2026.

/// A ball falling onto the ground in a 2D physics simulation with `rapier2d`.
mod rapier2d;
/// Runs the example.
/// Runs the Rapier 2D example on every supported desktop target.
fn main() {
    rapier2d::run();
}

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
