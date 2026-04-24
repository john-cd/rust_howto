# Linear Algebra

{{#include linear_algebra.incl.md}}

We will use two key crates:

- [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}}, a general-purpose linear algebra library with transformations and statically-sized or dynamically-sized matrices. However it supports only vectors (1d) and matrices (2d) and not higher-dimensional tensors.
- [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}} is less featureful than [`nalgebra`][c~nalgebra~docs]↗ but supports arbitrarily dimensioned arrays.

## Add Matrices {#adding-matrices}

[![ndarray][c~ndarray~docs~badge]][c~ndarray~docs] [![ndarray~crates.io][c~ndarray~crates.io~badge]][c~ndarray~crates.io] [![ndarray~repo][c~ndarray~repo~badge]][c~ndarray~repo] [![ndarray~lib.rs][c~ndarray~lib.rs~badge]][c~ndarray~lib.rs]{{hi:ndarray}}{{hi:Array}}{{hi:Blas}}{{hi:Data-structure}}{{hi:Matrix}}{{hi:Multidimensional}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~science][cat~science~badge]][cat~science]

This example creates two 2-D matrices with [`ndarray::arr2`][c~ndarray::arr2~docs]↗{{hi:ndarray::arr2}} and sums them element-wise.

Note that the sum is computed as `let sum = &a + &b`. The `&` operator is used to avoid consuming `a` and `b`, making them available later for display. A new array is created containing their sum.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/linear_algebra/add_matrices.rs:example}}
```

## Multiply Matrices {#multiplying-matrices}

[![ndarray][c~ndarray~docs~badge]][c~ndarray~docs] [![ndarray~crates.io][c~ndarray~crates.io~badge]][c~ndarray~crates.io] [![ndarray~repo][c~ndarray~repo~badge]][c~ndarray~repo] [![ndarray~lib.rs][c~ndarray~lib.rs~badge]][c~ndarray~lib.rs]{{hi:ndarray}}{{hi:Array}}{{hi:Blas}}{{hi:Data-structure}}{{hi:Matrix}}{{hi:Multidimensional}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~science][cat~science~badge]][cat~science]

This code example creates two matrices with [`ndarray::arr2`][c~ndarray::arr2~docs]↗{{hi:ndarray::arr2}} and performs matrix multiplication on them with [`ndarray::ArrayBase::dot`][c~ndarray::ArrayBase::dot~docs]↗{{hi:ndarray::ArrayBase::dot}}.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/linear_algebra/multiply_matrices.rs:example}}
```

## Multiply a Scalar with a Vector and a Matrix {#multiply-a-scalar-with-a-vector-and-a-matrix}

[![ndarray][c~ndarray~docs~badge]][c~ndarray~docs] [![ndarray~crates.io][c~ndarray~crates.io~badge]][c~ndarray~crates.io] [![ndarray~repo][c~ndarray~repo~badge]][c~ndarray~repo] [![ndarray~lib.rs][c~ndarray~lib.rs~badge]][c~ndarray~lib.rs]{{hi:ndarray}}{{hi:Array}}{{hi:Blas}}{{hi:Data-structure}}{{hi:Matrix}}{{hi:Multidimensional}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~science][cat~science~badge]][cat~science]

The following example creates a 1-D array (vector) with [`ndarray::arr1`][c~ndarray::arr1~docs]↗{{hi:ndarray::arr1}} and a 2-D array (matrix) with [`ndarray::arr2`][c~ndarray::arr2~docs]↗{{hi:ndarray::arr2}}.

First, a scalar is multiplied by the vector to get another vector. Then, the matrix is multiplied by the new vector with `ndarray::Array2::dot`↗ (Matrix multiplication is performed using `ndarray::Array2::dot`↗, while the `*` operator performs element-wise multiplication.)

In [`ndarray`][c~ndarray~docs]↗, 1-D arrays can be interpreted as either row or column vectors depending on context. If representing the orientation of a vector is important, a 2-D array with one row or one column must be used instead. In this example, the vector is a 1-D array on the right-hand side, so `ndarray::Array2::dot`↗ handles it as a column vector.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/linear_algebra/multiply_scalar_vector_matrix.rs:example}}
```

## Compare Vectors {#vector-comparison}

[![ndarray][c~ndarray~docs~badge]][c~ndarray~docs] [![ndarray~crates.io][c~ndarray~crates.io~badge]][c~ndarray~crates.io] [![ndarray~repo][c~ndarray~repo~badge]][c~ndarray~repo] [![ndarray~lib.rs][c~ndarray~lib.rs~badge]][c~ndarray~lib.rs]{{hi:ndarray}}{{hi:Array}}{{hi:Blas}}{{hi:Data-structure}}{{hi:Matrix}}{{hi:Multidimensional}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~science][cat~science~badge]][cat~science]

The [`ndarray`][c~ndarray~docs]↗ crate supports a number of ways to create arrays -- this recipe create [`ndarray::Array`][c~ndarray::Array~docs]↗{{hi:ndarray::Array}} from [`std::vec::Vec`][c~std::vec::Vec~docs]↗{{hi:std::vec::Vec}} using [`std::convert::From`][c~std::convert::From~docs]↗{{hi:std::convert::From}}. Then, it sums the arrays element-wise.

This recipe contains an example of comparing two floating-point [vectors][p~vectors] element-wise. Floating-point numbers are often stored inexactly, making exact comparisons difficult. However, the [`approx::assert_abs_diff_eq`][c~approx::assert_abs_diff_eq~docs]↗{{hi:approx::assert_abs_diff_eq}} macro from the [`approx`][c~approx~docs]↗{{hi:approx}} crate allows for convenient element-wise comparisons. To use the [`approx`][c~approx~docs]↗ crate with [`ndarray`][c~ndarray~docs]↗, the [`approx`][c~approx~docs]↗{{hi:approx}} feature must be added to the [`ndarray`][c~ndarray~docs]↗ dependency in [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}}. For example, `ndarray = { version = "0.15.6", features = [ "approx" ] }`.

This recipe also contains additional ownership examples. Here, `let z = a + b` consumes `a` and `b`, updates `a` with the result, then moves ownership to `z`. Alternatively,
`let w = &c + &d` creates a new vector without consuming `c` or `d`, allowing their modification later. See [Binary Operators With Two Arrays][c~ndarray::ArrayBase~docs]↗ for additional detail.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/linear_algebra/vector_comparison.rs:example}}
```

## Calculate Vector Norms {#vector-norm}

[![ndarray][c~ndarray~docs~badge]][c~ndarray~docs] [![ndarray~crates.io][c~ndarray~crates.io~badge]][c~ndarray~crates.io] [![ndarray~repo][c~ndarray~repo~badge]][c~ndarray~repo] [![ndarray~lib.rs][c~ndarray~lib.rs~badge]][c~ndarray~lib.rs]{{hi:ndarray}}{{hi:Array}}{{hi:Blas}}{{hi:Data-structure}}{{hi:Matrix}}{{hi:Multidimensional}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~science][cat~science~badge]][cat~science]

This recipe demonstrates use of the [`ndarray::Array1`][c~ndarray::Array1~docs]↗{{hi:ndarray::Array1}} type, [`ndarray::Array1`][c~ndarray::Array1~docs]↗{{hi:ndarray::Array1}} type,
[`ndarray::ArrayBase::fold`][c~ndarray::ArrayBase::fold~docs]↗{{hi:ndarray::ArrayBase::fold}} method, and [`ndarray::ArrayBase::dot`][c~ndarray::ArrayBase::dot~docs]↗{{hi:ndarray::ArrayBase::dot}} method in computing the [`l1`][wolfram~l1-norm]↗{{hi:L1 norm}} and [`l2`][wolfram~l2-norm]↗{{hi:L2 norm}} norms of a given vector.

The [`l2_norm`][wolfram~l2-norm]↗ function is the simpler of the two, as it computes the square root of the dot product of a vector with itself. The [`l1_norm`][wolfram~l1-norm]↗ function is computed by a [`ndarray::ArrayBase::fold`][c~ndarray::ArrayBase::fold~docs]↗{{hi:ndarray::ArrayBase::fold}} operation that sums the absolute values of the elements. (This could also be performed with `x.mapv(f64::abs).scalar_sum()`, but that would allocate a new array for the result of the `mapv`.)

Note that both [`l1_norm`][wolfram~l1-norm]↗ and [`l2_norm`][wolfram~l2-norm]↗ take the [`ndarray::ArrayView1`][c~ndarray::ArrayView1~docs]↗{{hi:ndarray::ArrayView1}} type. This recipe considers vector norms, so the norm functions only need to accept one-dimensional views, hence [`ndarray::ArrayView1`][c~ndarray::ArrayView1~docs]↗{{hi:ndarray::ArrayView1}}. While the functions could take a parameter of type `&Array1<f64>` instead, that would require the caller to have a reference to an owned array, which is more restrictive than just having access to a view (since a view can be created from any array or view, not just an owned array).

[`ndarray::Array`][c~ndarray::Array~docs]↗{{hi:ndarray::Array}} and [`ndarray::ArrayView`][c~ndarray::ArrayView~docs]↗ are both type aliases for [`ndarray::ArrayBase`][c~ndarray::ArrayBase~docs]↗. So, the most general argument type for the caller would be `&ArrayBase<S, Ix1> where S: Data`, because then the caller could use `&array` or `&view` instead of `x.view()`. If the function is part of a public API, that may be a better choice for the benefit of users. For internal functions, the more concise `ArrayView1<f64>` may be preferable.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/linear_algebra/vector_norm.rs:example}}
```

## Invert a Matrix {#invert-matrix}

[![nalgebra][c~nalgebra~docs~badge]][c~nalgebra~docs]{{hi:nalgebra}}
[![nalgebra~crates.io][c~nalgebra~crates.io~badge]][c~nalgebra~crates.io]
[![nalgebra~repo][c~nalgebra~repo~badge]][c~nalgebra~repo]
[![nalgebra~lib.rs][c~nalgebra~lib.rs~badge]][c~nalgebra~lib.rs]
[![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}
[![cat~science][cat~science~badge]][cat~science]
[![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}}

This code snippet creates a 3x3 matrix with [`nalgebra::Matrix3`][c~nalgebra::Matrix3~docs]↗{{hi:nalgebra::Matrix3}} and inverts it, if possible.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/linear_algebra/invert_matrix.rs:example}}
```

## Use nalgebra Vectors {#nalgebra-vectors}

[![nalgebra][c~nalgebra~docs~badge]][c~nalgebra~docs]{{hi:nalgebra}}
[![nalgebra~crates.io][c~nalgebra~crates.io~badge]][c~nalgebra~crates.io]
[![nalgebra~repo][c~nalgebra~repo~badge]][c~nalgebra~repo]
[![nalgebra~lib.rs][c~nalgebra~lib.rs~badge]][c~nalgebra~lib.rs]
[![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}}
[![cat~science][cat~science~badge]][cat~science]

[`nalgebra`][c~nalgebra~docs]↗ provides fixed-size and dynamically-sized vectors and matrices. This example creates two [`nalgebra::Vector3`][c~nalgebra::Vector3~docs]↗{{hi:nalgebra::Vector3}} values and demonstrates:

- **Dot product**: `v1.dot(&v2)` gives the scalar product of two vectors.
- **Cross product**: `v1.cross(&v2)` gives the vector perpendicular to both (3D only).
- **Norm**: `v.norm()` returns the Euclidean (L2) magnitude of the vector.
- **Normalization**: `v.normalize()` returns a unit vector in the same direction.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/linear_algebra/nalgebra_vectors.rs:example}}
```

## Apply Transformations {#nalgebra-transformations}

[![nalgebra][c~nalgebra~docs~badge]][c~nalgebra~docs]{{hi:nalgebra}}
[![nalgebra~crates.io][c~nalgebra~crates.io~badge]][c~nalgebra~crates.io]
[![nalgebra~repo][c~nalgebra~repo~badge]][c~nalgebra~repo]
[![nalgebra~lib.rs][c~nalgebra~lib.rs~badge]][c~nalgebra~lib.rs]
[![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}}
[![cat~science][cat~science~badge]][cat~science]

[`nalgebra`][c~nalgebra~docs]↗ provides types for geometric transformations. This example demonstrates:

- **[`nalgebra::Rotation3`][c~nalgebra::Rotation3~docs]↗{{hi:nalgebra::Rotation3}}**: creates a rotation around an axis by an angle using `Rotation3::from_axis_angle`.
- **[`nalgebra::Translation3`][c~nalgebra::Translation3~docs]↗{{hi:nalgebra::Translation3}}**: represents a translation in 3D space.
- **[`nalgebra::Isometry3`][c~nalgebra::Isometry3~docs]↗{{hi:nalgebra::Isometry3}}**: combines a rotation and a translation into a single rigid-body transformation.

The `*` operator applies the transformation to a [`nalgebra::Point3`][c~nalgebra::Point3~docs]↗{{hi:nalgebra::Point3}}.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/linear_algebra/nalgebra_transformations.rs:example}}
```

## Decompose a Matrix with SVD {#nalgebra-svd}

[![nalgebra][c~nalgebra~docs~badge]][c~nalgebra~docs]{{hi:nalgebra}}
[![nalgebra~crates.io][c~nalgebra~crates.io~badge]][c~nalgebra~crates.io]
[![nalgebra~repo][c~nalgebra~repo~badge]][c~nalgebra~repo]
[![nalgebra~lib.rs][c~nalgebra~lib.rs~badge]][c~nalgebra~lib.rs]
[![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}}
[![cat~science][cat~science~badge]][cat~science]

[Singular Value Decomposition (SVD)][wikipedia~singular-value-decomposition]↗{{hi:SVD}} factorizes a matrix `M` into `U * Σ * Vᵀ`, where `U` and `V` are orthogonal matrices and `Σ` is a diagonal matrix of singular values. SVD is widely used in dimensionality reduction, pseudoinverse computation, and solving least-squares problems.

This example uses [`nalgebra::Matrix3`][c~nalgebra::Matrix3~docs]↗{{hi:nalgebra::Matrix3}} and calls `.svd(true, true)` to compute the full decomposition. It also shows how to compute the determinant and trace of the matrix using `.determinant()` and `.trace()`.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/linear_algebra/nalgebra_decomposition.rs:example}}
```

## (De)serialize a Matrix {#deserialize-a-matrix}

[![nalgebra][c~nalgebra~docs~badge]][c~nalgebra~docs]{{hi:nalgebra}} [![nalgebra~crates.io][c~nalgebra~crates.io~badge]][c~nalgebra~crates.io] [![nalgebra~repo][c~nalgebra~repo~badge]][c~nalgebra~repo] [![nalgebra~lib.rs][c~nalgebra~lib.rs~badge]][c~nalgebra~lib.rs] [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~science][cat~science~badge]][cat~science]

You can serialize and deserialize a matrix to and from [[JSON]] using [`serde_json::to_string`][c~serde_json::to_string~docs]↗{{hi:serde_json::to_string}} and `serde_json::from_str`{{hi:serde_json::from_str}}.

Note that serialization followed by deserialization gives back the original matrix.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/linear_algebra/deserialize_matrix.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[vectors | Vectors]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

