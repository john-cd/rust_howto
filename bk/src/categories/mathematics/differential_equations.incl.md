<!-- TODO Reviewed ??-??-2026 -->

## Solve an Initial Value Problem {#solve-an-initial-value-problem}

An initial value problem describes how a state changes over an independent variable such as time, starting from a known state. The [`ode_solvers`][c~ode_solvers~docs]↗{{hi:ode_solvers}} crate provides fixed-step and adaptive Runge-Kutta methods. This example uses the fourth-order fixed-step Runge-Kutta method to approximate the solution of `dy/dt = -2y` with `y(0) = 1`.

The step size affects the approximation's accuracy. For a problem with a known exact solution, compare the numerical result against it as this test does.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/differential_equations/exponential_decay.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[numerical_analysis | Numerical Analysis]].
- [[simulation | Simulation]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [accuracy review: mathematics](https://github.com/john-cd/rust_howto/issues/1894)
</div>
