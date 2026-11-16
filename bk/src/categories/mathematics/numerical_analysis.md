<!-- TODO Reviewed ??-??-2026. -->

# Numerical Analysis

{{#include numerical_analysis.incl.md}}

## Find a Root with Brent's Method {#find-a-root-with-brents-method}

[![argmin][c~argmin~docs~badge]][c~argmin~docs]{{hi:argmin}} [![cat~science][cat~science~badge]][cat~science]

Brent's method combines bisection, the secant method, and inverse quadratic interpolation. It requires an interval whose endpoint values have opposite signs. This example finds the root of `cos(x) - x` in `[0, 1]`.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/numerical_analysis/root_finding.rs:example}}
```

## Compare Floating-Point Summation Methods

Adding floating-point values in a different order can change the result. [Taming Floating-Point Sums][blog~floating-point-sums]↗ compares ordinary accumulation, pairwise summation, and compensation, including how each approach handles rounding error.

This sequence has a known real-number sum of `1`: the large positive and negative terms cancel. Ordinary left-to-right `f64` addition loses the small term; Neumaier compensation retains it. The example rejects `NaN`, infinity, and a non-finite final result rather than assigning them a domain-specific meaning.

```rust
{{#include ../../../crates/cats/mathematics/examples/numerical_analysis/accurate_sum.rs:example}}
```

Run `cargo test -p mathematics --example accurate_sum` from `bk/crates`. Parallel reductions and SIMD may combine terms in another order and produce a different result; neither this example nor a single benchmark establishes a throughput advantage. [`std::arch`](https://doc.rust-lang.org/std/arch/) intrinsics are target-specific and require runtime feature detection or a target guarantee when necessary. The portable [`std::simd`](https://doc.rust-lang.org/std/simd/) API remains unstable on the Rust 1.100 toolchain, so use a stable library fallback when portability matters. For matrix operations rather than a scalar sum, see [`faer`][faer~linear-algebra~docs]↗ in [Linear Algebra](linear_algebra.md).

## Related Topics {#related-topics .skip}

- [[linear_algebra | Linear Algebra]].
- [[optimization | Optimization]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
