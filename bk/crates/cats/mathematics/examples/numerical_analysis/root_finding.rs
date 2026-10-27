// ANCHOR: example
use argmin::core::CostFunction;
use argmin::core::Error;
use argmin::core::Executor;
use argmin::core::State;
use argmin::solver::brent::BrentRoot;

#[derive(Clone, Copy)]
struct CosMinusX;

impl CostFunction for CosMinusX {
    type Output = f64;
    type Param = f64;

    fn cost(&self, x: &Self::Param) -> Result<Self::Output, Error> {
        Ok(x.cos() - x)
    }
}

/// Finds the root of `cos(x) - x` in the interval `[0, 1]`.
fn find_root() -> f64 {
    let result = Executor::new(CosMinusX, BrentRoot::new(0.0, 1.0, 1e-12))
        .configure(|state| state.param(0.5).max_iters(100))
        .run()
        .expect("Brent's method should find the bracketed root");

    result
        .state()
        .get_best_param()
        .copied()
        .expect("Brent's method should record its best root estimate")
}

/// Prints the computed root estimate.
fn main() {
    let root = find_root();
    println!("cos(x) - x = 0 at x ≈ {root:.12}");
}
// ANCHOR_END: example

/// Runs the example from the chapter's example runner.
pub fn run() {
    main();
}

/// Verifies Brent's method returns an accurate root estimate.
#[test]
fn test_brent_method_finds_root() {
    let root = find_root();
    assert!((root.cos() - root).abs() < 1e-10);
}
