//! Runs the simulation example.
//!
//! TODO Reviewed ??-??-2026.

/// Simulate a single-server bank queue with a timestamp-ordered event heap.
mod discrete_event;
/// Integrate exponential decay with a fixed-step fourth-order Runge-Kutta
mod ode_solver;

/// Runs the discrete-event and ODE simulation examples.
fn main() {
    discrete_event::run();
    ode_solver::run();
}
