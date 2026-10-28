#![cfg(any(target_os = "linux", target_os = "windows"))]
// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calls embedded Python code and passes it positional arguments with PyO3.

use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

/// Calls a Python function with and without arguments.
fn test_my_function() -> PyResult<()> {
    Python::attach(|py| {
        let fun: Py<PyAny> = PyModule::from_code(
            py,
            c_str!(
                "def example(*args, **kwargs):
                if args != ():
                    print('called with args', args)
                if kwargs != {}:
                    print('called with kwargs', kwargs)
                if args == () and kwargs == {}:
                    print('called with no arguments')"
            ),
            c_str!(""),
            c_str!(""),
        )?
        .getattr("example")?
        .unbind();

        // Call object without any arguments:
        fun.call0(py)?;

        // Pass object with Python tuple of positional arguments:
        let elements: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
        let tuple = PyTuple::new(py, elements)?;
        assert_eq!(format!("{tuple:?}"), "(0, 1, 2, 3, 4, 5)");
        fun.call1(py, tuple)?;

        Ok(())
    })
}

/// Runs a Python script and calls a Python function.
///
/// # Errors
///
/// Returns a Python error if interpreter initialization or a Python operation
/// fails.
fn main() -> PyResult<()> {
    // Initialize the embedded interpreter. This crate does not enable the
    // `auto-initialize` feature, so initialize it before attaching to Python.
    Python::initialize();

    // Run a Python script:
    Python::attach(|py| {
        py.run(pyo3::ffi::c_str!("print('Hello World')"), None, None)
    })?;

    test_my_function()
}
// ANCHOR_END: example

/// Runs the embedded Python example.
///
/// # Errors
///
/// Returns a Python error if interpreter initialization or a Python operation
/// fails.
pub fn run() -> PyResult<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the pyo3 example through its entry point.
    #[test]
    #[cfg_attr(
        target_os = "windows",
        ignore = "set PYTHONHOME to the Python base prefix and rerun this test"
    )]
    fn test_pyo3_example_entry_point() -> PyResult<()> {
        main()
    }
}
