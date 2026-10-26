<!-- TODO Reviewed ??-??-2026 -->

# Optimization

{{#include optimization.incl.md}}

## Minimize a Function with Brent's Method {#minimize-a-function-with-brents-method}

[![argmin][c~argmin~docs~badge]][c~argmin~docs]{{hi:argmin}} [![cat~science][cat~science~badge]][cat~science]

Brent's minimization method combines golden section search with parabolic interpolation. It searches within a bounded interval without requiring a derivative. This example finds the minimum of a quadratic function on `[-5, 8]`.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/optimization/brent_minimization.rs:example}}
```

## Minimize a Function with Nelder-Mead {#minimize-a-function-with-nelder-mead}

[![argmin][c~argmin~docs~badge]][c~argmin~docs]{{hi:argmin}} [![argmin-math][c~argmin-math~docs~badge]][c~argmin-math~docs]{{hi:argmin-math}} [![cat~science][cat~science~badge]][cat~science]

The Nelder-Mead simplex method needs no derivatives, so it suits smooth functions whose gradient is unavailable. This example minimizes the Rosenbrock function, a narrow curved valley with its minimum at `(1, 1)`. Gradient-based solvers in `argmin`, such as `LBFGS`, converge faster when you can supply gradients.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/optimization/nelder_mead_rosenbrock.rs:example}}
```

## Minimize a Function with CMA-ES {#minimize-a-function-with-cma-es}

[![cmaes][c~cmaes~docs~badge]][c~cmaes~docs]{{hi:cmaes}} [![cat~science][cat~science~badge]][cat~science]

CMA-ES (Covariance Matrix Adaptation Evolution Strategy) is a derivative-free method for black-box, non-convex problems. It samples candidates from a Gaussian whose shape adapts as the search proceeds. Fix the seed so tests are deterministic.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/optimization/cmaes_sphere.rs:example}}
```

## Minimize a Function with Particle Swarm Optimization {#minimize-a-function-with-particle-swarm-optimization}

[![rand][c~rand~docs~badge]][c~rand~docs]{{hi:rand}} [![cat~science][cat~science~badge]][cat~science]

A swarm of particles moves through the search space; each blends its momentum with a pull toward its own best position and the swarm's best position. The algorithm is short enough to write by hand with a seeded `StdRng`. The [`pso`](https://crates.io/crates/pso)↗ crate packages the same idea. This example minimizes the Rastrigin function, which has many local minima.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/optimization/particle_swarm.rs:example}}
```

## Maximize Fitness with a Genetic Algorithm {#maximize-fitness-with-a-genetic-algorithm}

[![radiate][c~radiate~docs~badge]][c~radiate~docs]{{hi:radiate}} [![cat~science][cat~science~badge]][cat~science]

A genetic algorithm evolves a population through selection, crossover and mutation. This example evolves a 32-bit string toward all ones (the "OneMax" problem) with `radiate`, using a scoped seed for reproducibility.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/optimization/genetic_algorithm.rs:example}}
```

## Choosing a Solver {#choosing-a-solver}

- Smooth, convex or nearly so, with gradients available: gradient-based solvers (`argmin`'s `LBFGS`, line searches).
- Smooth but no gradients, few variables: Nelder-Mead, or Brent's method in one dimension.
- Black-box, noisy, multimodal: CMA-ES, particle swarm, genetic [algorithms][p~category-algorithms]{{hi:algorithms}}.
- Discrete or combinatorial: genetic [algorithms][p~category-algorithms]{{hi:algorithms}}, or a specialized solver.

Always seed random number generators in tests, and cap iterations or generations so a poorly scaled problem cannot run forever.

## Related Topics {#related-topics .skip}

- [[numerical_analysis | Numerical Analysis]].
- [[linear_algebra | Linear Algebra]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  accuracy review: mathematics
</div>
