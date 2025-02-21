# Mathematics

[![cat-mathematics][cat-mathematics-badge]][cat-mathematics]{{hi:Mathematics}}

Crates with a mathematical aspect.

## Linear algebra

{{#include linear_algebra.incl.md}}

## Trigonometry

{{#include trigonometry.incl.md}}

## Complex numbers

{{#include complex_numbers.incl.md}}

## Statistics

{{#include statistics.incl.md}}

## Additional numeric types

{{#include additional_numeric_types.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ P1 final review](https://github.com/john-cd/rust_howto/issues/936)

## Key Concepts

- Linear algebra: Vectors, matrices, transformations.
- Statistics: Mean, variance, distributions, hypothesis testing.
- Number theory: Prime numbers, modular arithmetic, etc.
- Numerical computation: Algorithms for solving mathematical problems.
- Random number generation: Generating random numbers for simulations, etc.

## Choosing the right crate

- General-purpose linear algebra: `nalgebra` is a solid choice.
- Graphics and games linear algebra: `glam` is often preferred for its performance.
- Statistics: `statrs` is comprehensive.
- Numerical computing and data analysis: `ndarray` is essential.
- Random numbers: `rand` is the standard.

For most general-purpose math needs, `nalgebra` and `rand` are excellent starting points.For graphics or game development, `glam` is often preferred.For statistics, `statrs` is a good option.If you're doing numerical computing, `ndarray` is crucial.And for specialized areas like number theory or units of measurement, there are specific crates available.

## Linear Algebra

- `nalgebra`: A widely used and mature linear algebra library.Excellent for general-purpose linear algebra.
- `glam`: A fast and ergonomic linear algebra library designed for graphics and games. Often preferred in those contexts.
- `ultraviolet`: Another linear algebra library, with a focus on graphics.

## Statistics 2

- `statrs`: A comprehensive statistics library.
- `rand`: While primarily for random number generation, `rand` also provides some statistical distributions and functions.

## Numbers

- `num-bigint`: For arbitrary-precision integers.
- `num-rational`: For rational numbers.
- `num`: A crate that provides numeric traits and other utilities.

## Scientific Computing

- `ndarray`: For N-dimensional arrays.Essential for numerical computation and data analysis.
- `nalgebra-lapack`: Provides LAPACK bindings for linear algebra operations.

## Random Number Generation

- `rand`: A popular and widely used random number generator crate.

## Complex Numbers

- `num-complex`: For working with complex numbers.

## Units of Measurement

Link:

- `uom`: A type-safe dimensional analysis library.

## Optimization

- `argmin`: A pure Rust optimization library.

## Plotting and Graphing (Visualizing Math)

Link:

- `plotters`: A plotting library for creating charts and graphs.

</div>
