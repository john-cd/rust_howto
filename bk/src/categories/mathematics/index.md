<!-- TODO Reviewed ??-??-2026 -->

# Mathematics

[![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}}

This section collects Rust crates and runnable examples for mathematical and numerical programming. The chapter tables show the examples currently available.

Some commonly used crates are [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}} for linear algebra, [`glam`][c~glam~docs]↗{{hi:glam}} for graphics-oriented vector and matrix operations, [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}} for multidimensional arrays, and [`statrs`][c~statrs~docs]↗{{hi:statrs}} for statistical distributions and functions. See the [[randomness | Randomness]] chapter for random number generation.

## Trigonometry

{{#include trigonometry.incl.md}}

## Linear Algebra

For vectors, matrices, decompositions, and transformations, the examples use `nalgebra`. Other options include:

- [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}} provides statically sized and dynamically sized matrices and vectors.
- [`glam`][c~glam~docs]↗{{hi:glam}} provides vector, matrix, quaternion, and transformation types for graphics and games.
- [`ultraviolet`][c~ultraviolet~docs]↗{{hi:ultraviolet}} is another graphics-oriented math library.
- [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}} provides N-dimensional arrays.
- [`nalgebra-lapack`][c~nalgebra-lapack~docs]↗{{hi:nalgebra-lapack}} connects nalgebra to LAPACK routines.

### Tensors

For multidimensional arrays, see [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}}. The examples below focus on vectors and matrices; there is no dedicated tensor example yet.

{{#include linear_algebra.incl.md}}

## Probability and Statistics

For random variables, distributions, hypothesis testing, probabilistic models, stochastic processes, and statistical inference, use the following crates:

- [`statrs`][c~statrs~docs]↗{{hi:statrs}} provides statistical distributions and functions.

{{#include statistics.incl.md}}

### Random Number Generation

Use [`rand`][c~rand~docs]↗{{hi:rand}} for random number generation.

See the [[randomness | Randomness]] chapter.

## Additional Numeric Types

To work with numeric types beyond what is available in the standard library, use the following crates:

- [`num`][c~num~docs]↗{{hi:num}} provides numeric traits and other utilities.
- [`num-bigint`][c~num-bigint~docs]↗{{hi:num-bigint}} works with arbitrary-precision integers.
- [`num-rational`][c~num-rational~docs]↗{{hi:num-rational}} provides rational numbers.

### Embedded Rust Mathematics

- [`micromath`][c~micromath~repo]↗{{hi:micromath}}: Embedded Rust arithmetic, 2D/3D vector, and statistics library.

### Fractions

[![fraction][c~fraction~docs~badge]][c~fraction~docs] [![fraction~crates.io][c~fraction~crates.io~badge]][c~fraction~crates.io] [![fraction~repo][c~fraction~repo~badge]][c~fraction~repo] [![fraction~lib.rs][c~fraction~lib.rs~badge]][c~fraction~lib.rs]{{hi:fraction}}{{hi:Float}}{{hi:Numeric}}{{hi:Decimal}}{{hi:fraction}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

The [`fraction`][c~fraction~docs]↗{{hi:fraction}} crate provides fraction and decimal types with lossless arithmetic. Its API is still evolving; review its current documentation before adopting it.
{{#include additional_numeric_types.incl.md}}

### Complex Numbers

- [`num-complex`][c~num-complex~docs]↗{{hi:num-complex}} for working with complex numbers.

{{#include complex_numbers.incl.md}}

## Numerical Analysis

Numerical analysis covers methods such as integration, differentiation, and root finding. The [Numerical Analysis chapter][p~numerical-analysis] demonstrates root finding with [`argmin`][c~argmin~docs]↗{{hi:argmin}}.

## Optimization

For numerical optimization, the [Optimization chapter][p~optimization] demonstrates function minimization with [`argmin`][c~argmin~docs]↗{{hi:argmin}}, which provides an interface for a range of optimization algorithms.

Optimization problems include linear, nonlinear, and integer programming, as well as decision models.

## Differential Equations

This topic includes ordinary and partial differential equations, dynamical systems, stability analysis, and control systems.

- [`diffsol`][c~diffsol~docs]↗ provides solvers for ordinary differential equations and semi-explicit differential-algebraic equations.
- [`ode_solvers`][c~ode_solvers~docs]↗ provides numerical methods for ordinary differential equations.

## Graph Theory

For graph data structures and algorithms, see [`petgraph`][c~petgraph~docs]↗{{hi:petgraph}} and the [[graphs | Graphs]] chapter.

## Discrete Mathematics

This area includes combinatorics, set theory, and logic. [`bit-set`][c~bit-set~docs]↗{{hi:bit-set}} provides a set backed by a bit vector.

## Units of Measurement

- [`uom`][c~uom~docs]↗{{hi:uom}} provides type-safe quantities and units of measurement.

## Related Topics

- [[algorithms | Algorithms]].
- [[cryptography | Cryptography]].
- Plotting and Graphing: [[visualization | Visualization]].
- [[randomness | Randomness]].
- [[sorting | Sorting]].

## Applications

- [[classical_machine_learning | Classical Machine Learning]].
- [[deep_learning | Deep Learning]].
- [[science | Science]].
  - [[science_geo | Geoscience]].
  - [[science_neuroscience | Neuroscience]].
- [[science_robotics | Robotics]].
- [[search | Search]].
- [[simulation | Simulation]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [accuracy review: mathematics](https://github.com/john-cd/rust_howto/issues/1894)
  [accuracy review: decisions for the maintainer](https://github.com/john-cd/rust_howto/issues/1897)
</div>
