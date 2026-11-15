//! Runs the rocket simulation example.
//!
//! TODO Reviewed ??-??-2026.

/// A basic one-dimensional rocket launch simulation.
mod aero_simulation;
/// Runs the example.
/// Runs the aerospace simulation example.
fn main() {
    aero_simulation::run();
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
