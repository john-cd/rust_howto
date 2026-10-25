// ANCHOR: example
use cmaes::CMAESOptions;
use cmaes::DVector;
use cmaes::TerminationReason;

/// A sphere function shifted so its minimum (0) is at `(3, -2, 1)`.
fn shifted_sphere(x: &DVector<f64>) -> f64 {
    let center = [3.0, -2.0, 1.0];
    x.iter().zip(center).map(|(xi, ci)| (xi - ci).powi(2)).sum()
}

/// Minimizes the function with the CMA-ES evolution strategy and returns the
/// best point and value. A fixed seed makes the run reproducible.
fn minimize() -> Option<(Vec<f64>, f64)> {
    let mut cmaes_state = CMAESOptions::new(vec![0.0; 3], 1.0)
        .seed(42)
        .fun_target(1e-10)
        .max_generations(500)
        .build(shifted_sphere)
        .ok()?;
    let results = cmaes_state.run();
    // Stopping on the target means the optimum was reached.
    let reached = results
        .reasons
        .iter()
        .any(|reason| matches!(reason, TerminationReason::FunTarget));
    let best = results.overall_best?;
    reached.then(|| (best.point.as_slice().to_vec(), best.value))
}

/// Prints the best point found.
fn main() {
    match minimize() {
        Some((point, value)) => {
            println!("minimum at {point:.4?}, f = {value:.2e}");
        }
        None => println!("CMA-ES did not reach the target"),
    }
}
// ANCHOR_END: example

/// Runs the example from the chapter's example runner.
pub fn run() {
    main();
}

/// Verifies that CMA-ES finds the shifted sphere's minimum.
#[test]
fn test() {
    let (point, value) = minimize().expect("CMA-ES should reach the target");
    assert!(value < 1e-9);
    for (found, expected) in point.iter().zip([3.0, -2.0, 1.0]) {
        assert!((found - expected).abs() < 1e-3);
    }
}
