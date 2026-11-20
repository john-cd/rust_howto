// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Measures the cost of finding a separator near the end of a byte buffer.

/// Finds the byte offset of the first newline.
fn first_newline(input: &[u8]) -> Option<usize> {
    input.iter().position(|&byte| byte == b'\n')
}

/// Measures the scan over two buffer lengths, with setup outside the timer.
#[divan::bench(args = [64, 4096])]
fn scan_for_newline(bencher: divan::Bencher, len: usize) {
    let mut input = vec![b'x'; len];
    input[len - 1] = b'\n';

    bencher.bench_local(|| first_newline(divan::black_box(input.as_slice())));
}

/// Runs the registered benchmarks when Cargo builds this target as a benchmark.
fn main() {
    divan::main();
}
// ANCHOR_END: example
