// ANCHOR: example
//! Minimize a one-variable function with Brent's method.
//!
//! Optimization looks for the input that makes a function as small as possible,
//! for example the cheapest design or the lowest error of a model. Brent's
//! method finds the minimum of a one-variable function inside a bracket,
//! without needing derivatives.

use argmin::core::CostFunction;
use argmin::core::Error;
use argmin::core::Executor;
use argmin::core::State;
use argmin::solver::brent::BrentOpt;

#[derive(Clone, Copy)]
struct Quadratic;

impl CostFunction for Quadratic {
    type Output = f64;
    type Param = f64;

    fn cost(&self, x: &Self::Param) -> Result<Self::Output, Error> {
        Ok((x - 2.5).powi(2) + 4.0)
    }
}

/// Finds the minimum of the quadratic function within `[-5, 8]`.
fn minimize() -> f64 {
    let solver =
        BrentOpt::new(-5.0, 8.0).set_tolerance(f64::EPSILON.sqrt(), 1e-10);
    let result = Executor::new(Quadratic, solver)
        .configure(|state| state.param(0.0).max_iters(100))
        .run()
        .expect("Brent's method should minimize the bounded function");

    result
        .state()
        .get_best_param()
        .copied()
        .expect("Brent's method should record its best estimate")
}

/// Prints the minimum point and corresponding function value.
fn main() {
    let minimum = minimize();
    let value = (minimum - 2.5).powi(2) + 4.0;
    println!("minimum at x = {minimum:.8}, f(x) = {value:.8}");
}
// ANCHOR_END: example

/// Runs the example from the chapter's example runner.
pub fn run() {
    main();
}

/// Verifies Brent's method locates the minimum of a bounded quadratic.
#[test]
fn test_brent_method_finds_quadratic_minimum() {
    let minimum = minimize();
    assert!((minimum - 2.5).abs() < 1e-8);
    assert!(((minimum - 2.5).powi(2) + 4.0 - 4.0).abs() < 1e-12);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
