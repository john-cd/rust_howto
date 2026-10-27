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
