// #![allow(dead_code)]
// #![cfg(target_os = "linux")]
// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// // This example demonstrates how to embed Python code within a Rust
// // application using the `pyo3` crate. It showcases the ability to define
// // Python functions within Rust, call them, and pass arguments.
// // The example covers:
// // - Defining a Python function as a string.
// // - Calling the Python function from Rust with and without arguments.
// // - Passing a Python tuple as an argument from Rust.
// // - Running a simple Python script from Rust.
// // - Preparing Python for use in a free-threaded context.

// use pyo3::ffi::c_str;
// use pyo3::prelude::*;
// use pyo3::types::PyTuple;

// fn test_my_function() -> PyResult<()> {
//     Python::with_gil(|py| {
//         let fun: PyObject = PyModule::from_code(
//             py,
//             c_str!(
//                 "def example(*args, **kwargs):
//                 if args != ():
//                     print('called with args', args)
//                 if kwargs != {}:
//                     print('called with kwargs', kwargs)
//                 if args == () and kwargs == {}:
//                     print('called with no arguments')"
//             ),
//             c_str!(""),
//             c_str!(""),
//         )?
//         .getattr("example")?
//         .into();

//         // Call object without any arguments:
//         fun.call0(py)?;

//         // Pass object with Python tuple of positional arguments:
//         let elements: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
//         let tuple = PyTuple::new(py, elements)?;
//         assert_eq!(format!("{:?}", tuple), "(0, 1, 2, 3, 4, 5)");
//         fun.call1(py, tuple)?;

//         Ok(())
//     })
// }

// fn main() {
//     // Prepares the use of Python in a free-threaded context:
//     pyo3::prepare_freethreaded_python();

//     // Run a Python script:
//     println!(
//         "{:?}",
//         Python::with_gil(|py| py.run(
//             pyo3::ffi::c_str!("print('Hello World')"),
//             None,
//             None
//         ))
//     );

//     println!("{:?}", test_my_function());
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish; fix py examples](https://github.com/john-cd/rust_howto/issues/78)
