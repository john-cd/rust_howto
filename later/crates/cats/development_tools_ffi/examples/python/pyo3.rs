#![allow(dead_code)]
#![cfg(target_os = "linux")]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Calling Python from Rust with `pyo3`.

// This example demonstrates how to embed Python code within a Rust
// application using the `pyo3` crate. It showcases the ability to define
// Python functions within Rust, call them, and pass arguments.
// The example covers:
// - Defining a Python function as a string.
// - Calling the Python function from Rust with and without arguments.
// - Passing a Python tuple as an argument from Rust.
// - Running a simple Python script from Rust.
// - Initializing the interpreter before attaching the current thread.

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

pub fn run() -> PyResult<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the pyo3 example through its entry point.
    #[test]
    fn test_pyo3_example_entry_point() -> PyResult<()> {
        main()
    }
}
// [finish; fix py examples](https://github.com/john-cd/rust_howto/issues/78)
