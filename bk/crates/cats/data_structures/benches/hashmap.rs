// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Criterion benchmark comparing `std::collections::HashMap`,
//! `ahash::AHashMap`, and `rustc_hash::FxHashMap`.
//!
//! Hasher trade-offs:
//! - `std::collections::HashMap` uses SipHash by default, which protects
//!   against HashDoS attacks with attacker-controlled keys at the cost of
//!   higher hashing latency.
//! - `ahash::AHashMap` uses a fast, non-cryptographic algorithm that leverages
//!   CPU instructions like AES-NI when available, making it excellent for
//!   string keys and general use.
//! - `rustc_hash::FxHashMap` uses FxHash, a lightweight multiply-and-rotate
//!   hasher optimized for short numeric keys (e.g., AST node IDs or integer
//!   handles).

use std::collections::HashMap;
use std::hint::black_box;

use ahash::AHashMap;
use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use rustc_hash::FxHashMap;

/// Sample text snippet for word frequency counting.
const SAMPLE_TEXT: &str = "\
The quick brown fox jumps over the lazy dog. \
The fox was quick, but the dog was lazy. \
Every fast hasher has distinct trade-offs for string keys and integer keys.";

/// Counts word occurrences using standard `std::collections::HashMap`.
fn count_std(text: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

/// Counts word occurrences using `ahash::AHashMap`.
fn count_ahash(text: &str) -> AHashMap<&str, usize> {
    let mut counts = AHashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

/// Counts word occurrences using `rustc_hash::FxHashMap`.
fn count_fx(text: &str) -> FxHashMap<&str, usize> {
    let mut counts = FxHashMap::default();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

/// Benchmarks word counting performance across hasher implementations.
fn benchmark_word_counting(c: &mut Criterion) {
    let mut group = c.benchmark_group("word_counting");

    group.bench_function("std_siphash", |b| {
        b.iter(|| count_std(black_box(SAMPLE_TEXT)));
    });

    group.bench_function("ahash", |b| {
        b.iter(|| count_ahash(black_box(SAMPLE_TEXT)));
    });

    group.bench_function("fxhash", |b| {
        b.iter(|| count_fx(black_box(SAMPLE_TEXT)));
    });

    group.finish();
}

/// Benchmarks insertion and lookup performance for integer keys (`u64`).
fn benchmark_integer_keys(c: &mut Criterion) {
    let mut group = c.benchmark_group("integer_keys");

    group.bench_function("std_siphash", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for i in 0..black_box(1_000_u64) {
                map.insert(i, i * 2);
            }
            let mut sum = 0_u64;
            for i in 0..black_box(1_000_u64) {
                if let Some(val) = map.get(&i) {
                    sum += val;
                }
            }
            black_box(sum)
        });
    });

    group.bench_function("ahash", |b| {
        b.iter(|| {
            let mut map = AHashMap::new();
            for i in 0..black_box(1_000_u64) {
                map.insert(i, i * 2);
            }
            let mut sum = 0_u64;
            for i in 0..black_box(1_000_u64) {
                if let Some(val) = map.get(&i) {
                    sum += val;
                }
            }
            black_box(sum)
        });
    });

    group.bench_function("fxhash", |b| {
        b.iter(|| {
            let mut map = FxHashMap::default();
            for i in 0..black_box(1_000_u64) {
                map.insert(i, i * 2);
            }
            let mut sum = 0_u64;
            for i in 0..black_box(1_000_u64) {
                if let Some(val) = map.get(&i) {
                    sum += val;
                }
            }
            black_box(sum)
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_word_counting, benchmark_integer_keys);
criterion_main!(benches);
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Runs the hash map benchmark through its entry point.
    #[test]
    fn test_hashmap_bench_entry_point() {
        super::main();
    }
}
