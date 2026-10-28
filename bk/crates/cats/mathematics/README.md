<!-- TODO Reviewed ??-??-2026 -->

# Mathematics

This crate contains code examples for the **Mathematics** category on crates.io. It demonstrates integration with key ecosystem crates including `anyhow`, `approx`, `argmin`, `argmin-math`, `clap`, `cmaes`.

## Examples Covered

- **Additional Numeric Types** (`examples/additional_numeric_types/`): Demonstrates concepts like `big_integers`, `fixed_point`, `half_precision`, `num_bigint`, `num_traits`, `ordered_float`, `rational_numbers`, `rug`, `rust_decimal`, `typenum`.
- **Complex Numbers** (`examples/complex_numbers/`): Demonstrates concepts like `add_complex`, `create_complex`, `mathematical_functions`.
- **Differential Equations** (`examples/differential_equations/`): Demonstrates concepts like `exponential_decay`.
- **Discrete Mathematics** (`examples/discrete_mathematics/`): Demonstrates concepts like `set_operations`.
- **Graph Theory** (`examples/graph_theory/`): Demonstrates concepts like `cycle_detection`, `topological_levels`.
- **Linear Algebra** (`examples/linear_algebra/`): Demonstrates concepts like `add_matrices`, `deserialize_matrix`, `invert_matrix`, `multiply_matrices`, `multiply_scalar_vector_matrix`, `nalgebra_decomposition`, `nalgebra_transformations`, `nalgebra_vectors`, `vector_comparison`, `vector_norm`.
- **Numerical Analysis** (`examples/numerical_analysis/`): Demonstrates concepts like `root_finding`.
- **Numerical Integration** (`examples/numerical_integration/`): Demonstrates concepts like `quadrature`.
- **Optimization** (`examples/optimization/`): Demonstrates concepts like `brent_minimization`, `cmaes_sphere`, `genetic_algorithm`, `nelder_mead_rosenbrock`, `particle_swarm`.
- **Stats** (`examples/stats/`): Demonstrates concepts like `central_tendency`, `central_tendency1`, `central_tendency2`, `standard_deviation`.
- **Trigonometry** (`examples/trigonometry/`): Demonstrates concepts like `latitude_longitude`, `side_length`, `tan_sin_cos`.
- **Units** (`examples/units/`): Demonstrates concepts like `quantities_and_conversions`.

## Usage

To check or run examples in this crate:

```console
cargo check --package mathematics --examples
cargo test --package mathematics
```

Code examples correspond to chapters in `bk/src/categories/mathematics` or related book sections.
