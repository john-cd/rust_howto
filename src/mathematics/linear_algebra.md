# Linear Algebra

## Adding matrices

[![ndarray-badge]][ndarray] [![cat-science-badge]][cat-science]

Creates two 2-D matrices with [`ndarray::arr2`][ndarray::arr2] and sums them element-wise.

Note the sum is computed as `let sum = &a + &b`. The `&` operator is used to avoid consuming `a` and `b`, making them available later for display. A new array is created containing their sum.

```rust,editable
{{#include ../../deps/examples/add-matrices.rs}}
```

## Multiplying matrices

[![ndarray-badge]][ndarray] [![cat-science-badge]][cat-science]

Creates two matrices with [`ndarray::arr2`][ndarray::arr2] and performs matrix multiplication on them with [`ndarray::ArrayBase::dot`][ndarray::ArrayBase::dot]

```rust,editable
{{#include ../../deps/examples/multiply-matrices.rs}}
```

## Multiply a scalar with a vector with a matrix

[![ndarray-badge]][ndarray] [![cat-science-badge]][cat-science]

Creates a 1-D array (vector) with [`ndarray::arr1`][ndarray::arr1] and a 2-D array (matrix) with [`ndarray::arr2`][ndarray::arr2]

First, a scalar is multiplied by the vector to get another vector. Then, the matrix is multiplied by the new vector with
[`ndarray::Array2::dot`][ndarray::Array2::dot] (Matrix multiplication is performed using `dot`, while the `*` operator performs element-wise multiplication.)

In `ndarray`, 1-D arrays can be interpreted as either row or column vectors depending on context. If representing the orientation of a vector is important, a 2-D array with one row or one column must be used instead. In this example, the vector is a 1-D array on the right-hand side, so `dot` handles it as a column vector.

```rust,editable
{{#include ../../deps/examples/multiply-scalar-vector-matrix.rs}}
```

## Vector comparison

[![ndarray-badge]][ndarray]

The [`ndarray`][ndarray] crate supports a number of ways to create arrays -- this recipe creates
[`ndarray::Array`][ndarray::Array] from `std::Vec` using `from`. Then, it sums the arrays element-wise.

This recipe contains an example of comparing two floating-point vectors element-wise. Floating-point numbers are often stored inexactly, making exact comparisons difficult. However, the [`assert_abs_diff_eq!`][approx::assert_abs_diff_eq] macro from the [`approx`][approx] crate allows for convenient element-wise comparisons. To use the `approx` crate with `ndarray`, the `approx` feature must be added to the `ndarray` dependency in `Cargo.toml`. For example,
`ndarray = { version = "0.13", features = ["approx"]["approx"] }`.

This recipe also contains additional ownership examples. Here, `let z = a + b` consumes
`a` and `b`, updates `a` with the result, then moves ownership to `z`. Alternatively,
`let w = &c + &d` creates a new vector without consuming `c` or `d`, allowing their modification later. See [Binary Operators With Two Arrays][ndarray::ArrayBase] for additional detail.

```rust,editable
{{#include ../../deps/examples/vector-comparison.rs}}
```

## Vector norm

[![ndarray-badge]][ndarray]

This recipe demonstrates use of the [`Array1`][ndarray::Array1] type, [`ArrayView1`][ndarray::ArrayView1] type,
[`fold`][ndarray::ArrayBase::fold] method, and [`dot`][ndarray::ArrayBase::dot] method in computing the [`l1`][l1] and [`l2`][l2] norms of a given vector.

+ The `l2_norm` function is the simpler of the two, as it computes the square root of the dot product of a vector with itself. + The `l1_norm` function is computed by a `fold` operation that sums the absolute values of the elements. (This could also be performed with `x.mapv(f64::abs).scalar_sum()`, but that would allocate a new array for the result of the `mapv`.)

Note that both `l1_norm` and `l2_norm` take the [`ArrayView1`][ndarray::ArrayView1] type. This recipe considers vector norms, so the norm functions only need to accept one-dimensional views (hence [`ArrayView1`][ndarray::ArrayView1]. While the functions could take a parameter of type `&Array1<f64>` instead, that would require the caller to have a reference to an owned array, which is more restrictive than just having access to a view (since a view can be created from any array or view, not just an owned array).

`Array` and `ArrayView` are both type aliases for `ArrayBase`. So, the most general argument type for the caller would be `&ArrayBase<S, Ix1> where S: Data`, because then the caller could use `&array` or `&view` instead of `x.view()`. If the function is part of a public API, that may be a better choice for the benefit of users. For internal functions, the more concise `ArrayView1<f64>` may be preferable.

```rust,editable
{{#include ../../deps/examples/vector-norm.rs}}
```

## Invert matrix

[![nalgebra-badge]][nalgebra] [![cat-science-badge]][cat-science]

Creates a 3x3 matrix with [`nalgebra::Matrix3`][nalgebra::Matrix3] and inverts it, if possible.

```rust,editable
{{#include ../../deps/examples/invert-matrix.rs}}
```

## (De)-Serialize a Matrix

[![ndarray-badge]][ndarray] [![cat-science-badge]][cat-science]

Serialize and deserialize a matrix to and from JSON. Serialization is taken care of by [`serde_json::to_string`][serde_json::to_string] and [`serde_json::from_str`][serde_json::from_str] performs deserialization.

Note that serialization followed by deserialization gives back the original matrix.

```rust
{{#include ../../deps/examples/deserialize-matrix.rs}}
```

{{#include ../refs/link-refs.md}}
