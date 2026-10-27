// ANCHOR: example
//! Integrate exponential decay with a fixed-step fourth-order Runge-Kutta
//! solver.

use ode_solvers::Rk4;
use ode_solvers::System;
use ode_solvers::Vector1;

/// The system `dy/dt = -y` has the analytic solution `y(t) = exp(-t)`.
struct ExponentialDecay;

impl System<f64, Vector1<f64>> for ExponentialDecay {
    /// Computes the derivative of the exponential-decay state.
    fn system(
        &self,
        _time: f64,
        state: &Vector1<f64>,
        derivative: &mut Vector1<f64>,
    ) {
        derivative[0] = -state[0];
    }
}

/// Integrates from 0 to 1 and returns the final value.
fn solve_exponential_decay() -> f64 {
    let mut solver =
        Rk4::new(ExponentialDecay, 0.0, Vector1::new(1.0), 1.0, 0.1);
    solver
        .integrate()
        .expect("the exponential-decay system should integrate");
    let final_state = solver
        .y_out()
        .last()
        .expect("the RK4 solver should return its final state");
    final_state[0]
}

/// Runs the ODE solver example and prints the numerical result.
fn main() {
    let final_value = solve_exponential_decay();
    println!(
        "y(1) = {final_value:.6} (analytic: {:.6})",
        (-1.0_f64).exp()
    );
}
// ANCHOR_END: example

/// Runs the ODE solver example through its entry point.
pub fn run() {
    main()
}

#[cfg(test)]
mod tests {
    use super::solve_exponential_decay;

    /// Verifies that RK4 agrees with the analytic exponential-decay solution.
    #[test]
    fn rk4_matches_analytic_exponential_decay() {
        let actual = solve_exponential_decay();
        let expected = (-1.0_f64).exp();

        assert!((actual - expected).abs() < 1.0e-5);
    }
}
