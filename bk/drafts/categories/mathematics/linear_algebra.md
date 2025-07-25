# Linear Algebra

{{#include linear_algebra.incl.md}}

We will use two key crates:

- [`nalgebra`][c~nalgebra~docs]⮳{{hi:nalgebra}}{{hi:nalgebra}}, a general-purpose linear algebra library with transformations and statically-sized or dynamically-sized matrices. However it supports only vectors (1d) and matrices (2d) and not higher-dimensional tensors.
- [`ndarray`][c~ndarray~docs]⮳{{hi:ndarray}}{{hi:ndarray}} is less featureful than [`nalgebra`][c~nalgebra~docs]⮳{{hi:nalgebra}} but supports arbitrarily dimensioned arrays.

## Add Matrices {#adding-matrices}

[![ndarray][c~ndarray~docs~badge]][c~ndarray~docs]{{hi:ndarray}}
[![ndarray~crates.io][c~ndarray~crates.io~badge]][c~ndarray~crates.io]
[![ndarray~github][c~ndarray~github~badge]][c~ndarray~github]
[![ndarray~lib.rs][c~ndarray~lib.rs~badge]][c~ndarray~lib.rs]
[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}
[![cat~science][cat~science~badge]][cat~science]{{hi:Science}}

Creates two 2-D matrices with [`ndarray::arr2`][c~ndarray::arr2~docs]{{hi:ndarray::arr2}}⮳ and sums them element-wise.

Note that the sum is computed as `let sum = &a + &b`. The `&` operator is used to avoid consuming `a` and `b`, making them available later for display. A new array is created containing their sum.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/linear_algebra/add_matrices.rs:example}}
```

## Multiply Matrices {#multiplying-matrices}

[![ndarray][c~ndarray~docs~badge]][c~ndarray~docs]{{hi:ndarray}}
[![ndarray~crates.io][c~ndarray~crates.io~badge]][c~ndarray~crates.io]
[![ndarray~github][c~ndarray~github~badge]][c~ndarray~github]
[![ndarray~lib.rs][c~ndarray~lib.rs~badge]][c~ndarray~lib.rs]
[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}
[![cat~science][cat~science~badge]][cat~science]{{hi:Science}}

Creates two matrices with [`ndarray::arr2`][c~ndarray::arr2~docs]{{hi:ndarray::arr2}}⮳ and performs matrix multiplication on them with [`ndarray::ArrayBase::dot`][c~ndarray::ArrayBase::dot~docs]{{hi:ndarray::ArrayBase::dot}}⮳.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/linear_algebra/multiply_matrices.rs:example}}
```

## Multiply a Scalar with a Vector and a Matrix {#multiply-a-scalar-with-a-vector-and-a-matrix}

[![ndarray][c~ndarray~docs~badge]][c~ndarray~docs]{{hi:ndarray}}
[![ndarray~crates.io][c~ndarray~crates.io~badge]][c~ndarray~crates.io]
[![ndarray~github][c~ndarray~github~badge]][c~ndarray~github]
[![ndarray~lib.rs][c~ndarray~lib.rs~badge]][c~ndarray~lib.rs]
[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}
[![cat~science][cat~science~badge]][cat~science]{{hi:Science}}

Creates a 1-D array (vector) with [`ndarray::arr1`][c~ndarray::arr1~docs]{{hi:ndarray::arr1}}⮳ and a 2-D array (matrix) with [`ndarray::arr2`][c~ndarray::arr2~docs]{{hi:ndarray::arr2}}⮳.

First, a scalar is multiplied by the vector to get another vector. Then, the matrix is multiplied by the new vector with `ndarray::Array2::dot`⮳ (Matrix multiplication is performed using `ndarray::Array2::dot`⮳, while the `*` operator performs element-wise multiplication.)

In [`ndarray`][c~ndarray~docs]{{hi:ndarray}}⮳, 1-D arrays can be interpreted as either row or column vectors depending on context. If representing the orientation of a vector is important, a 2-D array with one row or one column must be used instead. In this example, the vector is a 1-D array on the right-hand side, so `ndarray::Array2::dot`⮳ handles it as a column vector.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/linear_algebra/multiply_scalar_vector_matrix.rs:example}}
```

## Compare Vectors {#vector-comparison}

[![ndarray][c~ndarray~docs~badge]][c~ndarray~docs]{{hi:ndarray}}
[![ndarray~crates.io][c~ndarray~crates.io~badge]][c~ndarray~crates.io]
[![ndarray~github][c~ndarray~github~badge]][c~ndarray~github]
[![ndarray~lib.rs][c~ndarray~lib.rs~badge]][c~ndarray~lib.rs]
[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}
[![cat~science][cat~science~badge]][cat~science]{{hi:Science}}

The [`ndarray`][c~ndarray~docs]{{hi:ndarray}}⮳ crate supports a number of ways to create arrays -- this recipe creates
[`ndarray::Array`][c~ndarray::Array~docs]{{hi:ndarray::Array}}⮳ from [`std::Vec`][c~std::vec::Vec~docs]⮳{{hi:std::Vec}} using [`std::convert::From`][c~std::convert::From~docs]{{hi:std::convert::From}}⮳. Then, it sums the arrays element-wise.

This recipe contains an example of comparing two floating-point [vectors][p~vectors] element-wise. Floating-point numbers are often stored inexactly, making exact comparisons difficult. However, the [`approx::assert_abs_diff_eq`][c~approx::assert_abs_diff_eq~docs]{{hi:approx::assert_abs_diff_eq}}⮳ macro from the [`approx`][c~approx~docs]{{hi:approx}}⮳ crate allows for convenient element-wise comparisons. To use the [`approx`][c~approx~docs]{{hi:approx}}⮳ crate with [`ndarray`][c~ndarray~docs]{{hi:ndarray}}⮳, the [`approx`][c~approx~docs]{{hi:approx}}⮳ feature must be added to the [`ndarray`][c~ndarray~docs]{{hi:ndarray}}⮳ dependency in [`Cargo.toml`][book~cargo~cargo-toml]⮳{{hi:Cargo.toml}}. For example,
`ndarray = { version = "0.15.6", features = [ "approx" ] }`.

This recipe also contains additional ownership examples. Here, `let z = a + b` consumes
`a` and `b`, updates `a` with the result, then moves ownership to `z`. Alternatively,
`let w = &c + &d` creates a new vector without consuming `c` or `d`, allowing their modification later. See [Binary Operators With Two Arrays][c~ndarray::ArrayBase~docs]⮳ for additional detail.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/linear_algebra/vector_comparison.rs:example}}
```

## Calculate Vector Norms {#vector-norm}

[![ndarray][c~ndarray~docs~badge]][c~ndarray~docs]{{hi:ndarray}}
[![ndarray~crates.io][c~ndarray~crates.io~badge]][c~ndarray~crates.io]
[![ndarray~github][c~ndarray~github~badge]][c~ndarray~github]
[![ndarray~lib.rs][c~ndarray~lib.rs~badge]][c~ndarray~lib.rs]
[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}
[![cat~science][cat~science~badge]][cat~science]{{hi:Science}}

This recipe demonstrates use of the [`ndarray::Array1`][c~ndarray::Array1~docs]{{hi:ndarray::Array1}}⮳ type, [`ndarray::Array1`][c~ndarray::Array1~docs]{{hi:ndarray::Array1}}⮳ type,
[`ndarray::ArrayBase::fold`][c~ndarray::ArrayBase::fold~docs]{{hi:ndarray::ArrayBase::fold}} method, and [`ndarray::ArrayBase::dot`][c~ndarray::ArrayBase::dot~docs]{{hi:ndarray::ArrayBase::dot}}⮳ method in computing the [`l1`][wolfram~l1-norm]{{hi:L1 norm}}⮳ and [`l2`][wolfram~l2-norm]{{hi:L2 norm}}⮳ norms of a given vector.

The [`l2_norm`][wolfram~l2-norm]{{hi:L2 norm}}⮳ function is the simpler of the two, as it computes the square root of the dot product of a vector with itself. + The [`l1_norm`][wolfram~l1-norm]{{hi:L1 norm}}⮳ function is computed by a [`ndarray::ArrayBase::fold`][c~ndarray::ArrayBase::fold~docs]{{hi:ndarray::ArrayBase::fold}}⮳ operation that sums the absolute values of the elements. (This could also be performed with `x.mapv(f64::abs).scalar_sum()`, but that would allocate a new array for the result of the `mapv`{{hi:mapv}}.)

Note that both [`l1_norm`][wolfram~l1-norm]{{hi:L1 norm}}⮳ and [`l2_norm`][wolfram~l2-norm]{{hi:L2 norm}}⮳ take the [`ndarray::ArrayView1`][c~ndarray::ArrayView1~docs]{{hi:ndarray::ArrayView1}}⮳ type. This recipe considers vector norms, so the norm functions only need to accept one-dimensional views, hence [`ndarray::ArrayView1`][c~ndarray::ArrayView1~docs]{{hi:ndarray::ArrayView1}}⮳. While the functions could take a parameter of type `&Array1<f64>` instead, that would require the caller to have a reference to an owned array, which is more restrictive than just having access to a view (since a view can be created from any array or view, not just an owned array).

[`ndarray::Array`][c~ndarray::Array~docs]{{hi:ndarray::Array}}⮳ and [`ndarray::Array`][c~ndarray::Array~docs]{{hi:ndarray::Array}}⮳ are both type aliases for [`ndarray::Array`][c~ndarray::Array~docs]{{hi:ndarray::Array}}⮳. So, the most general argument type for the caller would be `&ArrayBase<S, Ix1> where S: Data`, because then the caller could use `&array` or `&view` instead of `x.view()`. If the function is part of a public API, that may be a better choice for the benefit of users. For internal functions, the more concise `ArrayView1<f64>` may be preferable.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/linear_algebra/vector_norm.rs:example}}
```

## Invert a Matrix {#invert-matrix}

[![nalgebra][c~nalgebra~docs~badge]][c~nalgebra~docs]{{hi:nalgebra}}
[![nalgebra~crates.io][c~nalgebra~crates.io~badge]][c~nalgebra~crates.io]
[![nalgebra~github][c~nalgebra~github~badge]][c~nalgebra~github]
[![nalgebra~lib.rs][c~nalgebra~lib.rs~badge]][c~nalgebra~lib.rs]
[![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}
[![cat~science][cat~science~badge]][cat~science]{{hi:Science}}
[![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}}

Creates a 3x3 matrix with [`nalgebra::Matrix3`][c~nalgebra::Matrix3~docs]{{hi:nalgebra::Matrix3}}⮳ and inverts it, if possible.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/linear_algebra/invert_matrix.rs:example}}
```

## (De)serialize a Matrix {#deserialize-a-matrix}

[![ndarray][c~ndarray~docs~badge]][c~ndarray~docs]{{hi:ndarray}}
[![ndarray~crates.io][c~ndarray~crates.io~badge]][c~ndarray~crates.io]
[![ndarray~github][c~ndarray~github~badge]][c~ndarray~github]
[![ndarray~lib.rs][c~ndarray~lib.rs~badge]][c~ndarray~lib.rs]
[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}
[![cat~science][cat~science~badge]][cat~science]{{hi:Science}}

Serialize and deserialize a matrix to and from [JSON][p~json]. Serialization is taken care of by [`serde_json::to_string`][c~serde_json::to_string~docs]{{hi:serde_json::to_string}}⮳ and [`serde_json::to_string`][c~serde_json::to_string~docs]{{hi:serde_json::to_string}}⮳ performs deserialization.

Note that serialization followed by deserialization gives back the original matrix.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/linear_algebra/deserialize_matrix.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[linear_algebra: review; cover more of `nalgebra` NOW](https://github.com/john-cd/rust_howto/issues/408)
</div>
