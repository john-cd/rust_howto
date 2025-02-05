// ANCHOR: example

// Divan is a Rust benchmarking framework.

// Add the following to your project’s `Cargo.toml`:
// [dev-dependencies]
// divan = "0.1.17"

// [[bench]]
// name = "example"
// harness = false

// Run the benchmark with `cargo bench`

// When `harness` is set to false, you are responsible for defining a main()
// function to run tests and benchmarks.
fn main() {
    // Run registered benchmarks.
    divan::main();
}

// Define a `fibonacci` function and
// register it for benchmarking.
#[divan::bench]
// #[divan::bench(
//    max_time = 0.001, // seconds
//    sample_size = 64, // 64 × 84 = 5376
//)]
fn fibonacci() -> u64 {
    fn compute(n: u64) -> u64 {
        if n <= 1 {
            1
        } else {
            compute(n - 2) + compute(n - 1)
        }
    }

    compute(divan::black_box(10))
}
// Example adapted from https://nikolaivazquez.com/blog/divan/
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [P1](https://github.com/john-cd/rust_howto/issues/747) rewrite divan example; cover more attributes
// https://nikolaivazquez.com/blog/divan/
// https://github.com/nvzqz/divan/tree/v0.1.0/examples/benches
