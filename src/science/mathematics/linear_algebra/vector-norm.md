## Vector norm

[![ndarray-badge]][ndarray]

This recipe demonstrates use of the [`Array1`] type, [`ArrayView1`] type,
[`fold`] method, and [`dot`] method in computing the [l1] and [l2] norms of a
given vector.

+ The `l2_norm` function is the simpler of the two, as it computes the
square root of the dot product of a vector with itself.
+ The `l1_norm` function is computed by a `fold`
operation that sums the absolute values of the elements. (This could also be
performed with `x.mapv(f64::abs).scalar_sum()`, but that would allocate a new
array for the result of the `mapv`.)

Note that both `l1_norm` and `l2_norm` take the [`ArrayView1`] type. This recipe
considers vector norms, so the norm functions only need to accept one-dimensional
views (hence [`ArrayView1`]). While the functions could take a
parameter of type `&Array1<f64>` instead, that would require the caller to have
a reference to an owned array, which is more restrictive than just having access
to a view (since a view can be created from any array or view, not just an owned
array).

`Array` and `ArrayView` are both type aliases for `ArrayBase`. So, the most
general argument type for the caller would be `&ArrayBase<S, Ix1> where S: Data`,
because then the caller could use `&array` or `&view` instead of `x.view()`.
If the function is part of a public API, that may be a better choice for the
benefit of users. For internal functions, the more concise `ArrayView1<f64>`
may be preferable.

```rust,editable
{#include ../../../deps/examples/vector-norm.rs}
```

[`Array1`]: https://docs.rs/ndarray/*/ndarray/type.Array1.html
[`ArrayView1`]: https://docs.rs/ndarray/*/ndarray/type.ArrayView1.html
[`dot`]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#method.dot
[`fold`]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#method.fold
[l1]: http://mathworld.wolfram.com/L1-Norm.html
[l2]: http://mathworld.wolfram.com/L2-Norm.html
