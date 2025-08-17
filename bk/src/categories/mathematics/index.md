# Mathematics

[![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}}

Crates with a mathematical aspect.

For most general-purpose math needs, [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}} (linear algebra) and [`rand`][c~rand~docs]↗{{hi:rand}} (random numbers) are excellent starting points. For graphics or game development, [`glam`][c~glam~docs]↗{{hi:glam}} is often preferred. For statistics, [`statrs`][c~statrs~docs]↗{{hi:statrs}} is comprehensive.If you're doing numerical computing, [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}}. There are also specific crates available for specialized areas like number theory or units of measurement.

## Trigonometry

{{#include trigonometry.incl.md}}

## Linear Algebra

Vectors, matrices, eigenvalues, and eigenvectors. Key crates include:

- [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}}: A widely used and mature linear algebra library. Excellent for general-purpose linear algebra.
- [`glam`][c~glam~docs]↗{{hi:glam}}: A fast and ergonomic linear algebra library designed for graphics and games. Often preferred in those contexts.
- [`ultraviolet`][c~ultraviolet~docs]↗{{hi:ultraviolet}}: Another linear algebra library, with a focus on graphics.
- [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}}: For N-dimensional arrays.Essential for numerical computation and data analysis.
- [`nalgebra-lapack`][c~nalgebra-lapack~docs]↗{{hi:nalgebra-lapack}} provides LAPACK bindings for linear algebra operations.

{{#include linear_algebra.incl.md}}

## Probability and Statistics

Random variables, distributions, and hypothesis testing. Probabilistic models, stochastic processes, and statistical inference.

- [`statrs`][c~statrs~docs]↗{{hi:statrs}}: A comprehensive statistics library.
- [`rand`][c~rand~docs]↗{{hi:rand}}: While primarily for random number generation, [`rand`][c~rand~docs]↗{{hi:rand}} also provides some statistical distributions and functions.

{{#include statistics.incl.md}}

### Random Number Generation

Generating random numbers for simulations, cryptography, etc.

- [`rand`][c~rand~docs]↗{{hi:rand}}: A popular and widely used random number generator crate.

## Additional Numeric Types

- [`num-bigint`][c~num-bigint~docs]↗{{hi:num-bigint}} for arbitrary-precision integers.
- [`num-rational`][c~num-rational~docs]↗{{hi:num-rational}} for rational numbers.
- [`num`][c~num~docs]↗{{hi:num}}: A crate that provides numeric traits and other utilities.

{{#include additional_numeric_types.incl.md}}

### Complex Numbers

- [`num-complex`][c~num-complex~docs]↗{{hi:num-complex}} for working with complex numbers.

{{#include complex_numbers.incl.md}}

## Numerical Analysis

Numerical integration, differentiation, and root-finding.

- `rust-num`{{hi:rust-num}}: numerical traits and operations.
- [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}}: numerical computations.

## Optimization

Linear, nonlinear, and integer programming. Convex analysis. Game theory and decision-making models.

- [`argmin`][c~argmin~docs]↗{{hi:argmin}} is a pure Rust optimization library.

## Differential Equations

Ordinary and partial differential equations and dynamical systems.
Stability analysis, chaos theory, and control systems.

## Graph Theory

Graphs, networks, and shortest paths.

- [`petgraph`][c~petgraph~docs]↗{{hi:petgraph}} (graph data structures and algorithms).

## Discrete Mathematics

Combinatorics, set theory, and logic.

- [`bit-set`][c~bit-set~docs]↗{{hi:bit-set}} (bit manipulation), [`regex`][c~regex~docs]↗{{hi:regex}} (pattern matching).

## Units of Measurement

- [`uom`][c~uom~docs]↗{{hi:uom}}: A type-safe dimensional analysis library.

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
[in depth review; decide what to cover in examples](https://github.com/john-cd/rust_howto/issues/936)

- [micromath][c~micromath~github]↗:  Embedded Rust arithmetic, 2D/3D vector, and statistics library.

[fraction][fraction~crates.io].

[fraction~crates.io]: https://crates.io/crates/fraction

TENSORS:

rusten is a minimal, didactic implementation of tensors, or, more accurately, multi-dimensional arrays, in Rust. It mimics NumPy in terms of style and offers core functionalities such as unary, binary, and reduction operations, broadcasting, shape transformations, slicing, and more, all while prioritizing clarity. Unlike existing packages like ndarray, rusten doesn't aim for state-of-the-art performance or support for all use cases. Instead, it's targeted towards those seeking to gain a bare-bones understanding of multi-dimensional containers and related operations, without emphasizing performance or a comprehensive feature set. Coupled with Rust's intuitive syntax and clean memory management model, this means the rusten codebase is easy to explore and extend. It's also worth mentioning that, despite its compact codebase, rusten covers a surprisingly broad range of applications; in fact, with the major exception of convolutional networks, it can express the forward passes of most deep learning models, including transformers.

The motivation behind rusten is that truly understanding how tensor manipulation works under the hood helps fluency in tools like PyTorch. Although there are myriad from-scratch projects devoted to other aspects of deep learning pipelines - common architectures from scratch, machine learning algorithms from scratch, autodiff from scratch, ... - there don't seem to be any learner-oriented resources on how tensors are handled on a low level. A very rudimentary tensor structure is straightforward, but its complexity grows exponentially with the addition of modern features such as broadcasting, non-contiguous views, etc., and rusten's goal is to implement these processes as transparently and plainly as possible to aid interested students. The most similar project is the Tensor by Eureka Labs, but whereas that is more concerned with fine-grained C memory management and concentrates on one-dimensional vectors, rusten's focus is more on the aforementioned advanced array functionalities.

https://www.reddit.com/r/rust/comments/1iuqwts/rusten_a_minimal_didactic_implementation_of/

</div>
