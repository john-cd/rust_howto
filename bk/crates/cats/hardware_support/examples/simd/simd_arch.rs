#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A dot product with SIMD: `std::arch` intrinsics selected at run time, a
//! scalar fallback, and the portable `wide` crate.
//!
//! `std::simd` (portable SIMD) is nightly-only at the time of writing.
//!
//! SIMD pays off on long, contiguous numeric loops; measure with `criterion`
//! before and after. Note that SIMD changes the order of floating-point
//! additions, so results can differ in the last bits from the scalar version
//! (the fixtures below use small integers, which `f32` represents exactly).

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use wide::f32x4;

/// The reference implementation.
fn dot_scalar(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

/// SSE2 (always available on `x86_64`): four lanes at a time.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
fn dot_sse2(a: &[f32], b: &[f32]) -> f32 {
    let len = a.len().min(b.len());
    let chunks = len / 4;
    let mut lanes = [0.0_f32; 4];
    // SAFETY: `sse2` is enabled for this function; every load reads four
    // `f32` values at `i * 4`, which is in bounds because `i < len / 4`;
    // `_mm_loadu_ps` / `_mm_storeu_ps` accept unaligned pointers.
    unsafe {
        let mut acc = _mm_setzero_ps();
        for i in 0..chunks {
            let va = _mm_loadu_ps(a.as_ptr().add(i * 4));
            let vb = _mm_loadu_ps(b.as_ptr().add(i * 4));
            acc = _mm_add_ps(acc, _mm_mul_ps(va, vb));
        }
        _mm_storeu_ps(lanes.as_mut_ptr(), acc);
    }
    let tail = dot_scalar(&a[chunks * 4..len], &b[chunks * 4..len]);
    lanes.iter().sum::<f32>() + tail
}

/// AVX2: eight lanes at a time. Only call after checking for support.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
fn dot_avx2(a: &[f32], b: &[f32]) -> f32 {
    let len = a.len().min(b.len());
    let chunks = len / 8;
    let mut lanes = [0.0_f32; 8];
    // SAFETY: `avx2` is enabled for this function; every load reads eight
    // `f32` values at `i * 8`, in bounds because `i < len / 8`; the unaligned
    // load / store intrinsics have no alignment requirement.
    unsafe {
        let mut acc = _mm256_setzero_ps();
        for i in 0..chunks {
            let va = _mm256_loadu_ps(a.as_ptr().add(i * 8));
            let vb = _mm256_loadu_ps(b.as_ptr().add(i * 8));
            acc = _mm256_add_ps(acc, _mm256_mul_ps(va, vb));
        }
        _mm256_storeu_ps(lanes.as_mut_ptr(), acc);
    }
    let tail = dot_scalar(&a[chunks * 8..len], &b[chunks * 8..len]);
    lanes.iter().sum::<f32>() + tail
}

/// The portable version: `wide::f32x4` compiles on stable Rust for every
/// target (to SSE, NEON, WASM `simd128`, or scalar code).
fn dot_wide(a: &[f32], b: &[f32]) -> f32 {
    let mut acc = f32x4::ZERO;
    let mut ca = a.chunks_exact(4);
    let mut cb = b.chunks_exact(4);
    for (x, y) in ca.by_ref().zip(cb.by_ref()) {
        acc += f32x4::new([x[0], x[1], x[2], x[3]])
            * f32x4::new([y[0], y[1], y[2], y[3]]);
    }
    acc.reduce_add() + dot_scalar(ca.remainder(), cb.remainder())
}

/// A safe API: picks the best implementation at run time.
pub fn dot(a: &[f32], b: &[f32]) -> f32 {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            // SAFETY: we just checked that the CPU supports AVX2.
            return unsafe { dot_avx2(a, b) };
        }
        if is_x86_feature_detected!("sse2") {
            // SAFETY: we just checked that the CPU supports SSE2.
            return unsafe { dot_sse2(a, b) };
        }
    }
    // Other targets (or very old CPUs): the scalar fallback.
    dot_scalar(a, b)
}

/// Small integers as `f32`, from a fixed pseudo-random sequence.
fn fixture(len: usize, seed: u32) -> Vec<f32> {
    let mut state = seed;
    (0..len)
        .map(|_| {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            ((state >> 24) % 16) as f32
        })
        .collect()
}

/// Compares the implementations on a fixed input.
fn main() {
    let a = fixture(1_003, 1);
    let b = fixture(1_003, 2);

    let expected = dot_scalar(&a, &b);
    assert_eq!(dot(&a, &b), expected);
    assert_eq!(dot_wide(&a, &b), expected);
    println!("dot = {expected}");
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_all_lengths() {
        // Covers empty input, a tail shorter than a vector, and exact
        // multiples.
        for len in 0..40 {
            let a = fixture(len, 3);
            let b = fixture(len, 4);
            let expected = dot_scalar(&a, &b);
            assert_eq!(dot(&a, &b), expected, "dispatch, len {len}");
            assert_eq!(dot_wide(&a, &b), expected, "wide, len {len}");
            #[cfg(target_arch = "x86_64")]
            {
                // SAFETY: SSE2 is part of the `x86_64` baseline.
                assert_eq!(unsafe { dot_sse2(&a, &b) }, expected);
            }
        }
    }
}
