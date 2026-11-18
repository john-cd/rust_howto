<!-- TODO Reviewed ??-??-2026. -->

# Mathematics

[![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}}

This section collects Rust crates and runnable examples for mathematical and numerical programming. The chapter tables show the examples currently available.

Some commonly used crates are [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}} for linear algebra, [`glam`][c~glam~docs]↗{{hi:glam}} for graphics-oriented vector and matrix operations, [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}} for multidimensional arrays, [`sprs`][c~sprs~docs]↗{{hi:sprs}} for sparse matrices, and [`statrs`][c~statrs~docs]↗{{hi:statrs}} for statistical distributions and functions. See the [Randomness](../algorithms/random_value_generation.md) chapter for random number generation.

## Trigonometry

{{#include trigonometry.incl.md}}

## Linear Algebra

For vectors, matrices, decompositions, and transformations, the examples use `nalgebra`. Other options include:

- [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}} provides statically sized and dynamically sized matrices and vectors.
- [`glam`][c~glam~docs]↗{{hi:glam}} provides vector, matrix, quaternion, and transformation types for graphics and games.
- [`ultraviolet`][c~ultraviolet~docs]↗{{hi:ultraviolet}} is another graphics-oriented math library.
- [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}} provides N-dimensional arrays.
- [`nalgebra-lapack`][c~nalgebra-lapack~docs]↗{{hi:nalgebra-lapack}} connects nalgebra to LAPACK routines.

{{#include linear_algebra.incl.md}}

For rank-three arrays, see [Reduce a Tensor Along an Axis](linear_algebra.md#reduce-a-tensor-along-an-axis).

## Probability and Statistics

For random variables, distributions, hypothesis [testing][cat~development-tools::testing]↗{{hi:development-tools::testing}}, probabilistic models, stochastic processes, and statistical inference, use the following crates:

- [`statrs`][c~statrs~docs]↗{{hi:statrs}} provides statistical distributions and functions.

{{#include statistics.incl.md}}

### Random Number Generation

Use [`rand`][c~rand~docs]↗{{hi:rand}} for random number generation.

See the [Randomness](../algorithms/random_value_generation.md) chapter.

## Additional Numeric Types

To work with numeric types beyond what is available in the standard library, use the following crates:

- [`num`][c~num~docs]↗{{hi:num}} provides numeric traits and other utilities.
- [`num-bigint`][c~num-bigint~docs]↗{{hi:num-bigint}} works with arbitrary-precision integers.
- [`num-rational`][c~num-rational~docs]↗{{hi:num-rational}} provides rational numbers.

### Embedded Rust Mathematics

- [`micromath`][c~micromath~repo]↗{{hi:micromath}}: Embedded Rust arithmetic, 2D/3D vector, and statistics library.

### Fractions

{{#badges fraction data-structures="Data structures"}}{{hi:Float}}{{hi:Numeric}}{{hi:Decimal}}{{hi:fraction}}

The [`fraction`][c~fraction~docs]↗{{hi:fraction}} crate provides fraction and decimal types with lossless arithmetic. Its API is still evolving; review its current documentation before adopting it.
{{#include additional_numeric_types.incl.md}}

### Complex Numbers

- [`num-complex`][c~num-complex~docs]↗{{hi:num-complex}} for working with complex numbers.

{{#include complex_numbers.incl.md}}

## Numerical Analysis

Numerical analysis covers methods such as integration, differentiation, and root finding. The [Numerical Analysis chapter][p~numerical-analysis] demonstrates root finding with [`argmin`][c~argmin~docs]↗{{hi:argmin}}.

## Numerical Integration

The [Numerical Integration chapter][p~numerical-integration] demonstrates dependency-free composite trapezoidal and Simpson's rules for estimating definite integrals.

## Optimization

For numerical optimization, the [Optimization chapter][p~optimization] demonstrates function minimization with [`argmin`][c~argmin~docs]↗{{hi:argmin}}, which provides an interface for a range of optimization algorithms.

Optimization problems include linear, nonlinear, and integer programming, as well as decision models.

## Differential Equations

This topic includes ordinary and partial differential equations, dynamical systems, stability analysis, and control systems. The [Differential Equations chapter][p~differential-equations] demonstrates solving an initial value problem with [`ode_solvers`][c~ode_solvers~docs]↗{{hi:ode_solvers}}.

- [`diffsol`][c~diffsol~docs]↗ provides solvers for ordinary differential equations and semi-explicit differential-algebraic equations.
- [`ode_solvers`][c~ode_solvers~docs]↗ provides numerical methods for ordinary differential equations.

## Graph Theory

For graph data structures and algorithms, the [Graph Theory chapter][p~graph-theory] demonstrates DAG scheduling and cycle detection with [`petgraph`][c~petgraph~docs]↗{{hi:petgraph}}. See also the [[graphs | Graphs]] chapter.

## Discrete Mathematics

This area includes combinatorics, set theory, and logic. The [Discrete Mathematics chapter][p~discrete-mathematics] demonstrates set operations with [`std::collections::BTreeSet`][c~std::collections::BTreeSet~docs]↗{{hi:BTreeSet}}. For compact integer sets, [`bit-set`][c~bit-set~docs]↗{{hi:bit-set}} provides a set backed by a bit vector.

## Units of Measurement

The [Units of Measurement chapter][p~units] demonstrates dimensional types and conversions with [`uom`][c~uom~docs]↗{{hi:uom}}.

## Related Topics

- [[algorithms | Algorithms]].
- [Cryptography](../cryptography/index.md).
- Plotting and Graphing: [Visualization](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/visualization/index.md).
- [Randomness](../algorithms/random_value_generation.md).
- [[sorting | Sorting]].

## Applications

- [Classical Machine Learning](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/science/classical_machine_learning.md).
- [Deep Learning](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/science/deep_learning.md).
- [Science](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/science/index.md).
  - [Geoscience](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/science_geo/index.md).
  - [Neuroscience](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/science_neuroscience/index.md).
- [Robotics](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/science_robotics/index.md).
- [Search](../database/search.md).
- [Simulation](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/simulation/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
