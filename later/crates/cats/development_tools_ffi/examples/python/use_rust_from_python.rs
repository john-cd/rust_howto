#![allow(dead_code)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(clippy::useless_conversion)]
#![cfg(target_os = "linux")]
// ANCHOR: example
use pyo3::prelude::*;

/// This is a simple function that takes two 64-bit integers as input and
/// returns their sum.
///
/// # Arguments
///
/// * `a` - The first 64-bit integer.
/// * `b` - The second 64-bit integer.
///
/// # Returns
///
/// * `Result<i64, pyo3::PyErr>` - The sum of `a` and `b` if successful,
///   otherwise a `pyo3::PyErr`.
///
/// # Example
/// ```python
/// result = my_module.my_function(10, 20) # result will be 30
/// ```
#[pyfunction]
fn my_function(a: i64, b: i64) -> Result<i64, pyo3::PyErr> {
    // Your actual function logic here:
    Ok::<_, pyo3::PyErr>(a + b)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
///
/// # Arguments
///
/// * `module` - A mutable reference to the Python module.
///
/// # Returns
///
/// * `PyResult<()>` - Returns `Ok(())` if the module is successfully
///   initialized, otherwise a `PyErr`.
///
/// # Example
/// ```python
/// import my_module
/// ```
#[pymodule]
fn my_module(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(my_function, module)?)
}

fn main() {
    // TODO: Add code to build the Python module and demonstrate how to call it
    // from Python.
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
// [finish; fix](https://github.com/john-cd/rust_howto/issues/996)
