// #![allow(dead_code)]

// use pyo3::prelude::*;

// #[pyfunction]
// fn my_function(a: i64, b: i64) -> PyResult<i64> {
//     // Your actual function logic here
//     Ok(a + b)
// }

// #[cfg(test)]
// mod tests {
//     use pyo3::types::PyDict;

//     use super::*;

//     #[test]
//     fn test_my_function() {
//         pyo3::Python::with_gil(|py| {
//             let module = PyModule::new(py, "my_module").unwrap();
//             module.add_function(wrap_pyfunction!(my_function,
//         module)).unwrap();

//             let result =
//         PyDict::new(py).call(module.getattr("my_function").
//         unwrap(), (1, 2)).unwrap();

//             assert_eq!(result.get_item(py,
//         0).unwrap().extract::<i64>().unwrap(), 3); });
//     }
// }
