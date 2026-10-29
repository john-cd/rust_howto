//! Runs the simulation example.
//!
//! TODO Reviewed ??-??-2026.

mod discrete_event;
mod ode_solver;

/// Runs the discrete-event and ODE simulation examples.
fn main() {
    discrete_event::run();
    ode_solver::run();
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
