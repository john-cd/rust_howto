#![allow(dead_code)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(clippy::useless_conversion)]
#![cfg(target_os = "linux")]
// ANCHOR: example
// COMING SOON
// ANCHOR_END: example

use std::result::Result;

use pyo3::prelude::*;

#[pyfunction]
fn my_function(a: i64, b: i64) -> Result<i64, pyo3::PyErr> {
    // Your actual function logic here
    Ok::<_, pyo3::PyErr>(a + b)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn my_module(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(my_function, module)?)
}

fn main() {
    // [WIP finish; fix](https://github.com/john-cd/rust_howto/issues/996)
}

#[test]
fn test() {
    main();
}
