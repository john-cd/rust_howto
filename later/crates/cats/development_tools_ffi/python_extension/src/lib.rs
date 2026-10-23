//! A small Python extension module implemented in Rust with PyO3.

use pyo3::exceptions::PyOverflowError;
use pyo3::prelude::*;

/// Adds two signed 64-bit integers and exposes overflow as a Python exception.
///
/// # Errors
///
/// Returns `OverflowError` if the sum does not fit in an `i64`.
#[pyfunction]
fn add(left: i64, right: i64) -> PyResult<i64> {
    left.checked_add(right).ok_or_else(|| {
        PyOverflowError::new_err("sum does not fit in a signed 64-bit integer")
    })
}

/// Registers the example function in the importable Python module.
///
/// # Errors
///
/// Returns a Python error if adding the function to the module fails.
#[pymodule]
fn rust_howto_python_module(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(add, module)?)
}
