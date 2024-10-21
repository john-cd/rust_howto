// TODO solve
// error: linking with `cc` failed: exit status: 1
//   = note: /usr/bin/ld: cannot find -lpython3.11: No such file or
// directory           collect2: error: ld returned 1 exit status

// #![allow(dead_code)]

// use pyo3::prelude::*;
// use pyo3::types::PyTuple;

// #[pyfunction]
// fn my_function(a: i64, b: i64) -> PyResult<i64> {
//     // Your actual function logic here
//     Ok(a + b)
// }

// #[pymodule]
// fn my_module(module: &Bound<'_, PyModule>) -> PyResult<()> {
//     module.add_function(wrap_pyfunction!(my_function, module)?)
// }

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn test_my_function() -> PyResult<()> {
//         let arg1 = "arg1";
//         let arg2 = "arg2";
//         let arg3 = "arg3";

//         Python::with_gil(|py| {
//             let fun: Py<PyAny> = PyModule::from_code_bound(
//                 py,
//                 "def example(*args, **kwargs):
//                 if args != ():
//                     print('called with args', args)
//                 if kwargs != {}:
//                     print('called with kwargs', kwargs)
//                 if args == () and kwargs == {}:
//                     print('called with no arguments')",
//                 "",
//                 "",
//             )?
//             .getattr("example")?
//             .into();

//             // call object without any arguments
//             fun.call0(py)?;

//             // pass object with Rust tuple of positional arguments
//             let args = (arg1, arg2, arg3);
//             fun.call1(py, args)?;

//             // call object with Python tuple of positional
// arguments             let args = PyTuple::new_bound(py, &[arg1,
// arg2, arg3]);             fun.call1(py, args)?;
//             Ok(())
//         })
//     }
// }
