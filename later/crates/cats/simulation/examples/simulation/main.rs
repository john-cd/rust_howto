//! Runs the simulation example.
//!
//! TODO Reviewed ??-??-2026.

/// Simulate a single-server bank queue with a timestamp-ordered event heap.
mod discrete_event;
/// Integrate exponential decay with a fixed-step fourth-order Runge-Kutta
mod ode_solver;
/// Runs the example.
/// Runs the discrete-event and ODE simulation examples.

fn main() {
    discrete_event::run();
    ode_solver::run();
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
