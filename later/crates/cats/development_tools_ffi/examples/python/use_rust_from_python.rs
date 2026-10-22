//! Demonstrates how to define a PyO3 Python module in Rust, register it with
//! `sys.modules`, and invoke its functions from embedded Python code.
//!
//! TODO Reviewed ??-??-2026

#![cfg(target_os = "linux")]

// ANCHOR: example
use pyo3::prelude::*;

/// Adds two 64-bit integers and returns their sum.
///
/// # Arguments
///
/// * `a` - The first 64-bit integer.
/// * `b` - The second 64-bit integer.
///
/// # Errors
///
/// Returns `PyOverflowError` when the sum does not fit in an `i64`.
///
/// # Example
///
/// ```python
/// result = my_module.my_function(10, 20)  # result will be 30
/// ```
#[pyfunction]
fn my_function(a: i64, b: i64) -> PyResult<i64> {
    a.checked_add(b).ok_or_else(|| {
        pyo3::exceptions::PyOverflowError::new_err(
            "integer addition overflowed i64",
        )
    })
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
///
/// # Arguments
///
/// * `module` - A mutable reference to the Python module.
///
/// # Errors
///
/// Returns `PyResult<()>` if adding functions or initializing the module fails.
///
/// # Example
///
/// ```python
/// import my_module
/// ```
#[pymodule]
fn my_module(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(my_function, module)?)
}

/// Initializes the Python interpreter, registers the Rust module `my_module` in
/// `sys.modules`, and executes Python code that imports and calls the module.
///
/// # Errors
///
/// Returns a `pyo3::PyErr` if Python initialization, module creation, or Python
/// script execution fails.
fn main() -> PyResult<()> {
    // Initialize the Python interpreter.
    Python::initialize();

    Python::attach(|py| {
        // Create the Rust-defined module.
        let module = PyModule::new(py, "my_module")?;
        my_module(&module)?;

        // Register the module in sys.modules so Python code can import it.
        let sys = py.import("sys")?;
        let modules = sys.getattr("modules")?;
        modules.set_item("my_module", module)?;

        // Execute Python code that imports and calls the Rust module.
        let code = pyo3::ffi::c_str!(
            "import my_module\n\
             result = my_module.my_function(10, 20)\n\
             assert result == 30, f'Expected 30, got {result}'\n\
             print(f'10 + 20 = {result}')"
        );
        py.run(code, None, None)?;

        Ok(())
    })
}
// ANCHOR_END: example

/// Entry point for running the example.
///
/// # Errors
///
/// Returns a `pyo3::PyErr` if `main` fails.
pub fn run() -> PyResult<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that `main` runs successfully and executes the embedded Python
    /// script.
    #[test]
    fn test_main() -> PyResult<()> {
        main()
    }

    /// Verifies that adding values outside the `i64` range returns a Python
    /// error.
    #[test]
    fn test_addition_overflow_returns_error() {
        assert!(my_function(i64::MAX, 1).is_err());
    }
}
// [finish; fix](https://github.com/john-cd/rust_howto/issues/996)
