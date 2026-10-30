// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Criterion benchmark comparing non-cryptographic and cryptographic hash
//! functions on byte buffers.
//!
//! Hasher categories:
//! - Fast non-cryptographic hashers (`xxhash-rust` XXH3, `foldhash`, `ahash`,
//!   `rustc-hash`) are optimized for speed and throughput when hashing
//!   in-memory data, checksums, or map keys.
//! - Standard library `DefaultHasher` (SipHash) provides HashDoS mitigation for
//!   general-purpose maps.
//! - Cryptographic hashers (`blake3`, `sha2`) provide strong collision
//!   resistance and preimage resistance for signatures and tamper-evident
//!   digests. BLAKE3 additionally utilizes SIMD parallelism.

use std::hash::BuildHasher;
use std::hash::Hasher;
use std::hint::black_box;

use ahash::AHasher;
use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use foldhash::quality::FixedState;
use rustc_hash::FxHasher;
use sha2::Digest;
use sha2::Sha256;
use xxhash_rust::xxh3::xxh3_64;

/// Generates a test buffer of `len` bytes filled with a deterministic pattern.
fn make_test_buffer(len: usize) -> Vec<u8> {
    (0..len).map(|i| (i % 256) as u8).collect()
}

/// Hashes a byte slice with standard `DefaultHasher` (SipHash 1-3).
fn hash_std(data: &[u8]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hasher.write(data);
    hasher.finish()
}

/// Hashes a byte slice with `AHasher`.
fn hash_ahash(data: &[u8]) -> u64 {
    let mut hasher = AHasher::default();
    hasher.write(data);
    hasher.finish()
}

/// Hashes a byte slice with `FxHasher`.
fn hash_fx(data: &[u8]) -> u64 {
    let mut hasher = FxHasher::default();
    hasher.write(data);
    hasher.finish()
}

/// Hashes a byte slice with `foldhash`.
fn hash_fold(data: &[u8]) -> u64 {
    FixedState::default().hash_one(data)
}

/// Benchmarks hashing performance on a small buffer (64 bytes).
fn benchmark_small_buffer(c: &mut Criterion) {
    let data = make_test_buffer(64);
    let mut group = c.benchmark_group("hashing_64_bytes");

    group.bench_function("std_siphash", |b| {
        b.iter(|| hash_std(black_box(&data)));
    });

    group.bench_function("ahash", |b| {
        b.iter(|| hash_ahash(black_box(&data)));
    });

    group.bench_function("fxhash", |b| {
        b.iter(|| hash_fx(black_box(&data)));
    });

    group.bench_function("foldhash", |b| {
        b.iter(|| hash_fold(black_box(&data)));
    });

    group.bench_function("xxh3_64", |b| {
        b.iter(|| xxh3_64(black_box(&data)));
    });

    group.bench_function("blake3", |b| {
        b.iter(|| blake3::hash(black_box(&data)));
    });

    group.bench_function("sha256", |b| {
        b.iter(|| Sha256::digest(black_box(&data)));
    });

    group.finish();
}

/// Benchmarks hashing performance on a medium buffer (64 KB).
fn benchmark_medium_buffer(c: &mut Criterion) {
    let data = make_test_buffer(64 * 1024);
    let mut group = c.benchmark_group("hashing_64KB_buffer");

    group.bench_function("std_siphash", |b| {
        b.iter(|| hash_std(black_box(&data)));
    });

    group.bench_function("ahash", |b| {
        b.iter(|| hash_ahash(black_box(&data)));
    });

    group.bench_function("fxhash", |b| {
        b.iter(|| hash_fx(black_box(&data)));
    });

    group.bench_function("foldhash", |b| {
        b.iter(|| hash_fold(black_box(&data)));
    });

    group.bench_function("xxh3_64", |b| {
        b.iter(|| xxh3_64(black_box(&data)));
    });

    group.bench_function("blake3", |b| {
        b.iter(|| blake3::hash(black_box(&data)));
    });

    group.bench_function("sha256", |b| {
        b.iter(|| Sha256::digest(black_box(&data)));
    });

    group.finish();
}

criterion_group!(benches, benchmark_small_buffer, benchmark_medium_buffer);
criterion_main!(benches);
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Runs the hashing benchmark through its entry point.
    #[test]
    fn test_hashing_bench_entry_point() {
        super::main();
    }
}
