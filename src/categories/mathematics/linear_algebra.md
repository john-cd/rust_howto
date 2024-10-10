# Linear Algebra

{{#include linear_algebra.incl.md}}

## Adding matrices

[![ndarray][c-ndarray-badge]][c-ndarray]  [![cat-science][cat-science-badge]][cat-science]

Creates two 2-D matrices with [`{{i:ndarray::arr2}}`][c-ndarray::arr2]⮳ and sums them element-wise.

Note the sum is computed as `let sum = &a + &b`. The `&` operator is used to avoid consuming `a` and `b`, making them available later for display. A new array is created containing their sum.

```rust,editable
{{#include ../../../deps/tests/add-matrices.rs}}
```

## Multiplying matrices

[![ndarray][c-ndarray-badge]][c-ndarray]  [![cat-science][cat-science-badge]][cat-science]

Creates two matrices with [`{{i:ndarray::arr2}}`][c-ndarray::arr2]⮳ and performs matrix multiplication on them with [`{{i:ndarray::ArrayBase::dot}}`][c-ndarray::ArrayBase::dot]⮳.

```rust,editable
{{#include ../../../deps/tests/multiply-matrices.rs}}
```

## Multiply a scalar with a vector with a matrix

[![ndarray][c-ndarray-badge]][c-ndarray]  [![cat-science][cat-science-badge]][cat-science]

Creates a 1-D array (vector) with [`{{i:ndarray::arr1}}`][c-ndarray::arr1]⮳ and a 2-D array (matrix) with [`{{i:ndarray::arr2}}`][c-ndarray::arr2]⮳

First, a scalar is multiplied by the vector to get another vector. Then, the matrix is multiplied by the new vector with
[`{{i:ndarray::Array2::dot}}`][c-ndarray::Array2::dot]⮳ (Matrix multiplication is performed using [`{{i:dot}}`][c-ndarray::Array2::dot]⮳, while the `*` operator performs element-wise multiplication.)

In [`{{i:ndarray}}`][c-ndarray]⮳, 1-D arrays can be interpreted as either row or column vectors depending on context. If representing the orientation of a vector is important, a 2-D array with one row or one column must be used instead. In this example, the vector is a 1-D array on the right-hand side, so [`{{i:dot}}`][c-ndarray::Array2::dot]⮳ handles it as a column vector.

```rust,editable
{{#include ../../../deps/tests/multiply-scalar-vector-matrix.rs}}
```

## Vector comparison

[![ndarray][c-ndarray-badge]][c-ndarray]

The [`{{i:ndarray}}`][c-ndarray]⮳ crate supports a number of ways to create arrays -- this recipe creates
[`{{i:ndarray::Array}}`][c-ndarray::Array]⮳ from `std::Vec` using [`{{i:from}}`][c-std::convert::From]⮳. Then, it sums the arrays element-wise.

This recipe contains an example of comparing two floating-point vectors element-wise. Floating-point numbers are often stored inexactly, making exact comparisons difficult. However, the [`{{i:assert_abs_diff_eq!}}`][c-approx::assert_abs_diff_eq]⮳ macro from the [`{{i:approx}}`][c-approx]⮳ crate allows for convenient element-wise comparisons. To use the [`{{i:approx}}`][c-approx]⮳ crate with [`{{i:ndarray}}`][c-ndarray]⮳, the [`{{i:approx}}`][c-approx]⮳ feature must be added to the [`{{i:ndarray}}`][c-ndarray]⮳ dependency in `Cargo.toml`. For example,
`ndarray = { version = "0.13", features = ["approx"]["approx"] }`.

This recipe also contains additional ownership examples. Here, `let z = a + b` consumes
`a` and `b`, updates `a` with the result, then moves ownership to `z`. Alternatively,
`let w = &c + &d` creates a new vector without consuming `c` or `d`, allowing their modification later. See [Binary Operators With Two Arrays][c-ndarray::ArrayBase]⮳ for additional detail.

```rust,editable
{{#include ../../../deps/tests/vector-comparison.rs}}
```

## Vector norm

[![ndarray][c-ndarray-badge]][c-ndarray]

This recipe demonstrates use of the [`{{i:Array1}}`][c-ndarray::Array1]⮳ type, [`{{i:ArrayView1}}`][c-ndarray::Array1]⮳ type,
[`{{i:fold}}`][c-ndarray::ArrayBase::fold] method, and [`{{i:dot}}`][c-ndarray::ArrayBase::dot]⮳ method in computing the [`{{i:l1}}`][l1-norm]⮳ and [`{{i:l2}}`][l2-norm]⮳ norms of a given vector.

+ The [`{{i:l2_norm}}`][l2-norm]⮳ function is the simpler of the two, as it computes the square root of the dot product of a vector with itself. + The [`{{i:l1_norm}}`][l1-norm]⮳ function is computed by a [`{{i:fold}}`][c-ndarray::ArrayBase::fold]⮳ operation that sums the absolute values of the elements. (This could also be performed with `x.mapv(f64::abs).scalar_sum()`, but that would allocate a new array for the result of the `{{i:mapv}}`.)

Note that both [`{{i:l1_norm}}`][l1-norm]⮳ and [`{{i:l2_norm}}`][l2-norm]⮳ take the [`{{i:ArrayView1}}`][c-ndarray::ArrayView1]⮳ type. This recipe considers vector norms, so the norm functions only need to accept one-dimensional views, hence [`{{i:ArrayView1}}`][c-ndarray::ArrayView1]⮳. While the functions could take a parameter of type `&Array1<f64>` instead, that would require the caller to have a reference to an owned array, which is more restrictive than just having access to a view (since a view can be created from any array or view, not just an owned array).

[`{{i:Array}}`][c-ndarray::Array]⮳ and [`{{i:ArrayView}}`][c-ndarray::Array]⮳ are both type aliases for [`{{i:ArrayBase}}`][c-ndarray::Array]⮳. So, the most general argument type for the caller would be `&ArrayBase<S, Ix1> where S: Data`, because then the caller could use `&array` or `&view` instead of `x.view()`. If the function is part of a public API, that may be a better choice for the benefit of users. For internal functions, the more concise `ArrayView1<f64>` may be preferable.

```rust,editable
{{#include ../../../deps/tests/vector-norm.rs}}
```

## Invert matrix

[![nalgebra][c-nalgebra-badge]][c-nalgebra]  [![cat-science][cat-science-badge]][cat-science]

Creates a 3x3 matrix with [`{{i:nalgebra::Matrix3}}`][c-nalgebra::Matrix3]⮳ and inverts it, if possible.

```rust,editable
{{#include ../../../deps/tests/invert-matrix.rs}}
```

## (De)-Serialize a Matrix

[![ndarray][c-ndarray-badge]][c-ndarray]  [![cat-science][cat-science-badge]][cat-science]

Serialize and deserialize a matrix to and from JSON. Serialization is taken care of by [`{{i:serde_json::to_string}}`][c-serde_json::to_string]⮳ and [`{{i:serde_json::from_str}}`][c-serde_json::to_string]⮳ performs deserialization.

Note that serialization followed by deserialization gives back the original matrix.

```rust
{{#include ../../../deps/tests/deserialize-matrix.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
