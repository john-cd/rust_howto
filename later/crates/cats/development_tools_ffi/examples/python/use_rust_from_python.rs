#![cfg(any(target_os = "linux", target_os = "windows"))]
// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates how to define a PyO3 Python module in Rust, register it with
//! `sys.modules`, and invoke its Rust function and class from embedded Python.
//!
//! PyO3 lets you write Python extension modules in Rust, which are fast and can
//! use threads. Tools such as `maturin` build the module into a wheel that
//! Python installs with `pip`.

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

/// A Python-visible counter backed by a Rust integer.
#[pyclass]
struct Counter {
    /// Current counter value.
    value: i64,
}

/// Methods exposed on Python's `Counter` class.
#[pymethods]
impl Counter {
    /// Creates a counter initialized to `value`.
    #[new]
    fn new(value: i64) -> Self {
        Self { value }
    }

    /// Returns the current counter value.
    #[getter]
    fn current_value(&self) -> i64 {
        self.value
    }

    /// Adds `amount` and returns the updated value.
    ///
    /// # Errors
    ///
    /// Returns `PyOverflowError` when the new value does not fit in an `i64`.
    fn increment(&mut self, amount: i64) -> PyResult<i64> {
        let next_value = self.value.checked_add(amount).ok_or_else(|| {
            pyo3::exceptions::PyOverflowError::new_err(
                "counter value overflowed i64",
            )
        })?;
        self.value = next_value;
        Ok(next_value)
    }
}

/// A Python module implemented in Rust that registers a function and class.
/// The module name is used when adding it to `sys.modules` below.
///
/// # Arguments
///
/// * `module` - A mutable reference to the Python module.
///
/// # Errors
///
/// Returns a Python error if registration of the function or class fails.
///
/// # Example
///
/// ```python
/// import my_module
/// ```
#[pymodule]
fn my_module(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(my_function, module)?)?;
    module.add_class::<Counter>()?;
    Ok(())
}

/// Initializes the Python interpreter, registers the Rust module `my_module` in
/// `sys.modules`, and executes Python code that imports and calls its function
/// and class.
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
             counter = my_module.Counter(5)\n\
             assert counter.current_value == 5\n\
             assert counter.increment(3) == 8\n\
             assert counter.current_value == 8\n\
             print(f'10 + 20 = {result}; counter = {counter.current_value}')"
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
    use std::error::Error;
    use std::path::Path;
    use std::process::Command;

    use super::*;

    /// Runs the use rust from python example through its entry point.
    #[test]
    #[cfg_attr(
        target_os = "windows",
        ignore = "set PYTHONHOME to the Python base prefix and rerun this test"
    )]
    fn test_use_rust_from_python_example_entry_point() -> PyResult<()> {
        main()
    }

    /// Verifies that adding values outside the `i64` range returns a Python
    /// error.
    #[test]
    fn test_addition_overflow_returns_error() {
        assert!(my_function(i64::MAX, 1).is_err());
    }

    /// Verifies that the Python-visible counter updates and returns its value.
    #[test]
    fn counter_increment_updates_value() -> PyResult<()> {
        let mut counter = Counter::new(5);
        assert_eq!(counter.increment(3)?, 8);
        assert_eq!(counter.current_value(), 8);
        Ok(())
    }

    /// Verifies that overflowing a counter returns a Python error.
    #[test]
    fn counter_increment_overflow_returns_error() {
        let mut counter = Counter::new(i64::MAX);
        assert!(counter.increment(1).is_err());
    }

    /// Builds the native extension and calls it from a separate Python process.
    #[test]
    #[ignore = "needs maturin, Python, and an active Python virtual environment"]
    fn builds_and_imports_the_python_extension() -> Result<(), Box<dyn Error>> {
        if std::env::var_os("VIRTUAL_ENV").is_none() {
            return Err("activate a Python virtual environment before running this test".into());
        }

        let extension_dir =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("python_extension");
        let maturin_status = Command::new("maturin")
            .arg("develop")
            .current_dir(&extension_dir)
            .status()?;
        if !maturin_status.success() {
            return Err(format!(
                "maturin develop failed with {maturin_status}"
            )
            .into());
        }

        let python_status = Command::new("python")
            .arg("python/use_rust_from_python.py")
            .current_dir(&extension_dir)
            .status()?;
        if !python_status.success() {
            return Err(format!(
                "the Python extension example failed with {python_status}"
            )
            .into());
        }

        Ok(())
    }
}
