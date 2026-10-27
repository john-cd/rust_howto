// ANCHOR: example
use argmin::core::CostFunction;
use argmin::core::Error;
use argmin::core::Executor;
use argmin::core::State;
use argmin::solver::neldermead::NelderMead;

/// The Rosenbrock "banana" function, whose global minimum is 0 at `(1, 1)`.
struct Rosenbrock;

impl CostFunction for Rosenbrock {
    type Output = f64;
    type Param = Vec<f64>;

    fn cost(&self, p: &Self::Param) -> Result<Self::Output, Error> {
        let (x, y) = (p[0], p[1]);
        Ok((1.0 - x).powi(2) + 100.0 * (y - x * x).powi(2))
    }
}

/// Minimizes the Rosenbrock function with the derivative-free Nelder-Mead
/// simplex method, starting from a simplex of three points around `(-1.2, 1)`.
fn minimize() -> Result<(Vec<f64>, f64), Error> {
    let simplex = vec![vec![-1.2, 1.0], vec![-1.0, 1.0], vec![-1.2, 0.8]];
    let solver = NelderMead::new(simplex).with_sd_tolerance(1e-10)?;
    let result = Executor::new(Rosenbrock, solver)
        .configure(|state| state.max_iters(500))
        .run()?;
    let state = result.state();
    let best = state
        .get_best_param()
        .cloned()
        .ok_or_else(|| Error::msg("the solver recorded no best parameter"))?;
    Ok((best, state.get_best_cost()))
}

/// Prints the minimum found.
fn main() -> Result<(), Error> {
    let (best, cost) = minimize()?;
    println!("minimum at {best:.5?}, f = {cost:.3e}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the example from the chapter's example runner.
pub fn run() -> Result<(), Error> {
    main()
}

/// Verifies that Nelder-Mead converges to the Rosenbrock minimum `(1, 1)`.
#[test]
fn test_nelder_mead_finds_rosenbrock_minimum() -> Result<(), Error> {
    let (best, cost) = minimize()?;
    assert!((best[0] - 1.0).abs() < 1e-3, "x = {}", best[0]);
    assert!((best[1] - 1.0).abs() < 1e-3, "y = {}", best[1]);
    assert!(cost < 1e-6);
    Ok(())
}
