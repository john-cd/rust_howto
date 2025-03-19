// ANCHOR: example
use criterion::Criterion;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;

// Define a simple fibonacci function.
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// The `benchmark_fibonacci` function benchmarks the performance of the
// fibonacci function with the input `20`.
fn benchmark_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci 20", |b| b.iter(|| fibonacci(black_box(20))));
}

// Use criterion_group! and criterion_main! macros to specify and run the
// benchmarks.
criterion_group!(benches, benchmark_fibonacci);
criterion_main!(benches);

// Run the benchmarks with `cargo bench`
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
