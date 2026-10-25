// ANCHOR: example
use anyhow::Context;
use anyhow::Result;
use nalgebra::Vector1;
use ode_solvers::Rk4;
use ode_solvers::System;

/// Models exponential decay with the differential equation `dy/dt = -2y`.
struct ExponentialDecay;

impl System<f64, Vector1<f64>> for ExponentialDecay {
    /// Computes the derivative of the state at the given time.
    fn system(
        &self,
        _time: f64,
        state: &Vector1<f64>,
        derivative: &mut Vector1<f64>,
    ) {
        derivative[0] = -2.0 * state[0];
    }
}

/// Integrates exponential decay from time zero to one with a fixed step size.
fn solve() -> Result<f64> {
    let mut solver =
        Rk4::new(ExponentialDecay, 0.0, Vector1::new(1.0), 1.0, 0.1);
    solver
        .integrate()
        .context("could not integrate the decay equation")?;
    solver
        .y_out()
        .last()
        .map(|state| state[0])
        .context("the solver returned no state values")
}

/// Prints the final state computed by the Runge-Kutta solver.
fn main() {
    let final_value =
        solve().expect("the sample initial value problem should integrate");
    println!("y(1) ≈ {final_value:.6}");
}
// ANCHOR_END: example

/// Runs the example from the chapter's example runner.
pub fn run() {
    main();
}

/// Verifies the fixed-step solver approximates the exact exponential solution.
#[test]
fn test() {
    let final_value =
        solve().expect("the sample initial value problem should integrate");
    assert!((final_value - (-2.0_f64).exp()).abs() < 1e-5);
}
