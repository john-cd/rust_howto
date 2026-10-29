//! Runs the rocket simulation example.
//!
//! TODO Reviewed ??-??-2026.

mod aero_simulation;

/// Runs the aerospace simulation example.
fn main() {
    aero_simulation::run();
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
