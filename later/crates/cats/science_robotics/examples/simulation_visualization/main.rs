//! Runs the robotics simulation and visualization examples.
//!
//! TODO Reviewed ??-??-2026.

mod robot_simulation;

/// Runs the headless Rapier robot simulation example.
fn main() {
    robot_simulation::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
