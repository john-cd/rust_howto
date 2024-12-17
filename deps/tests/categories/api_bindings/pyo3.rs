#![allow(dead_code)]
#![allow(unsafe_op_in_unsafe_fn)]
// ANCHOR: example
use pyo3::prelude::*;
use pyo3::types::PyTuple;

#[pyfunction]
fn my_function(a: i64, b: i64) -> PyResult<i64> {
    // Your actual function logic here
    Ok(a + b)
}

#[pymodule]
fn my_module(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(my_function, module)?)
}

fn test_my_function() -> PyResult<()> {
    let arg1 = "arg1";
    let arg2 = "arg2";
    let arg3 = "arg3";

    Python::with_gil(|py| {
        let fun: Py<PyAny> = PyModule::from_code_bound(
            py,
            "def example(*args, **kwargs):
                if args != ():
                    print('called with args', args)
                if kwargs != {}:
                    print('called with kwargs', kwargs)
                if args == () and kwargs == {}:
                    print('called with no arguments')",
            "",
            "",
        )?
        .getattr("example")?
        .into();

        // call object without any arguments
        fun.call0(py)?;

        // pass object with Rust tuple of positional arguments
        let args = (arg1, arg2, arg3);
        fun.call1(py, args)?;

        // call object with Python tuple of positional arguments
        let args = PyTuple::new_bound(py, [arg1, arg2, arg3]);
        fun.call1(py, args)?;
        Ok(())
    })
}

fn main() {
    pyo3::prepare_freethreaded_python();
    println!("{:?}", test_my_function());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}

// [pyo3: ^ edition 2024 migration ()](https://github.com/john-cd/rust_howto/issues/143)
// [fix pyo3.rs](https://github.com/john-cd/rust_howto/issues/78)
