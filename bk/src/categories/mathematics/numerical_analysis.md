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

## Related Topics {#related-topics .skip}

- [[linear_algebra | Linear Algebra]].
- [[optimization | Optimization]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
