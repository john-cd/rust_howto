//! Runs the robotics simulation and visualization examples.
//!
//! TODO Reviewed ??-??-2026.

/// Drop a robot chassis onto a floor with Rapier's headless 2D physics
mod robot_simulation;
/// Runs the example.
/// Runs the headless Rapier robot simulation example.

fn main() {
    robot_simulation::run();
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
