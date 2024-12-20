#![allow(dead_code)]
#![allow(unsafe_op_in_unsafe_fn)]
// ANCHOR: example
use pyo3::prelude::*;

#[pyfunction]
fn my_function(a: i64, b: i64) -> PyResult<i64> {
    // Your actual function logic here
    Ok(a + b)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn my_module(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(my_function, module)?)
}

fn main() {
    // TODO P0
}

// ANCHOR_END: example

#[test]
fn test() {
    main();
}
