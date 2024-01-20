## Vector comparison

[![ndarray-badge]][ndarray]

The [ndarray] crate supports a number of ways to create arrays -- this recipe creates
[`ndarray::Array`]s from `std::Vec` using `from`. Then, it sums the arrays element-wise.

This recipe contains an example of comparing two floating-point vectors element-wise.
Floating-point numbers are often stored inexactly, making exact comparisons difficult.
However, the [`assert_abs_diff_eq!`] macro from the [`approx`] crate allows for convenient
element-wise comparisons. To use the `approx` crate with `ndarray`, the `approx`
feature must be added to the `ndarray` dependency in `Cargo.toml`. For example,
`ndarray = { version = "0.13", features = ["approx"] }`.

This recipe also contains additional ownership examples. Here, `let z = a + b` consumes
`a` and `b`, updates `a` with the result, then moves ownership to `z`. Alternatively,
`let w = &c + &d` creates a new vector without consuming `c` or `d`, allowing
their modification later. See [Binary Operators With Two Arrays] for additional detail.

```rust,editable
{#include ../../../deps/examples/vector-comparison.rs}
```

[`approx`]: https://docs.rs/approx/*/approx/index.html
[`assert_abs_diff_eq!`]: https://docs.rs/approx/*/approx/macro.assert_abs_diff_eq.html
[Binary Operators With Two Arrays]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#binary-operators-with-two-arrays
[ndarray]: https://docs.rs/crate/ndarray/*
[`ndarray::Array`]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html
