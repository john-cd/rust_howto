<!-- TODO Reviewed ??-??-2026 -->

# Numerical Integration

{{#include numerical_integration.incl.md}}

Numerical integration approximates the definite integral of a function using a finite set of samples. This original recipe was prompted by a [Rust Cookbook proposal][rust-cookbook~numerical-integration-issue].

## Composite Trapezoidal and Simpson's Rules {#composite-trapezoidal-and-simpsons-rules}

Both composite rules divide a finite interval `[a, b]` into `n` equally spaced subintervals with width `h = (b - a) / n`. The trapezoidal rule joins neighboring samples with straight lines; Simpson's 1/3 rule fits parabolas through pairs of subintervals.

The trapezoidal rule requires a positive subinterval count. Simpson's rule additionally requires an even count. The integrand should be reasonably smooth over the interval for the usual convergence rates; neither method estimates its own error or handles discontinuities and singularities automatically. Reversing the bounds reverses the sign of the integral.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/numerical_integration/quadrature.rs:example}}
```

For sufficiently smooth functions, the composite trapezoidal rule has a leading global error proportional to `h²`, while Simpson's rule has a leading error proportional to `h⁴`. Halving `h` therefore often reduces the truncation error by about four or sixteen times, respectively, while truncation error dominates. Floating-point rounding and cancellation eventually limit the benefit of smaller steps; compare successive refinements and use a known integral to check an implementation. This example rejects non-finite bounds and sample values, as well as a step size or estimate that cannot be represented as a finite `f64`.

## Related Topics {#related-topics .skip}

- [[numerical_analysis | Numerical Analysis]].
- [[differential_equations | Differential Equations]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
