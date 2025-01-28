// // ANCHOR: example
// COMING SOON
// // ANCHOR_END: example

// fn main() {
//     // Run registered benchmarks.
//     divan::main();
// }

// // Define a `fibonacci` function and
// // register it for benchmarking.
// #[divan::bench]
// // #[divan::bench(
// //    max_time = 0.001, // seconds
// //    sample_size = 64, // 64 × 84 = 5376
// //)]
// fn fibonacci() -> u64 {
//     fn compute(n: u64) -> u64 {
//         if n <= 1 {
//             1
//         } else {
//             compute(n - 2) + compute(n - 1)
//         }
//     }

//     compute(divan::black_box(10))
// }

// #[test]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/747) rewrite divan example
// // https://nikolaivazquez.com/blog/divan/
