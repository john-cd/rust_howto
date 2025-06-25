// // ANCHOR: example
// //! `divan` is a Rust benchmarking framework.
// //!
// //! To use `divan`, add the following to your project's `Cargo.toml`:
// //!
// //! ```toml
// //! [dev-dependencies]
// //! divan = "0.1.17"
// //!
// //! [[bench]]
// //! name = "example" # Update as needed.
// //! harness = false
// //! ```
// //!
// //! Run the benchmark with `cargo bench`.
// //!
// //! This example demonstrates how to benchmark a simple `fibonacci` function.

// /// Define a `fibonacci` function to benchmark.
// fn compute_fibonacci(n: u64) -> u64 {
//     if n <= 1 {
//         1
//     } else {
//         compute_fibonacci(n - 2) + compute_fibonacci(n - 1)
//     }
// }

// mod bench {
//     use super::*;

//     /// Register the function above for benchmarking.
//     #[divan::bench]
//     // #[divan::bench(
//     //    max_time = 0.001, // seconds
//     //    sample_size = 64, //
//     //)]
//     fn fibonacci() -> u64 {
//         compute_fibonacci(divan::black_box(10))
//     }
// }

// /// When `harness` is set to false, you are responsible for defining a main()
// /// function to run tests and benchmarks.
// fn main() {
//     // Run registered benchmarks.
//     #[cfg(not(test))]
//     divan::main();
// }
// // Example adapted from https://nikolaivazquez.com/blog/divan/
// // ANCHOR_END: example

// // [finish. add to md: https://nikolaivazquez.com/blog/divan/](https://github.com/john-cd/rust_howto/issues/747) cover more attributes; cover blockbox / bencher
// // review <https://github.com/nvzqz/divan/tree/v0.1.0/examples/benches>
fn main() {}
