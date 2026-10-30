<!-- TODO Reviewed ??-??-2026 -->

# Other Simulation Tools

{{#include simulation.incl.md}}

## Discrete-Event Simulation {#simul}

[![NeXosim][c~nexosim~docs~badge]][c~nexosim~docs]

A discrete-event simulator advances directly from one scheduled event to the next. The example uses a min-priority [`BinaryHeap`][std~collections~BinaryHeap] to model customer arrivals and service completions without a runtime or network. For larger event-driven models, [NeXosim][c~nexosim~docs] has a v1.0 release and supports component-based simulation benches; review its [release notes][nexosim~releases] when choosing an API version.

The bank has one server and a first-in, first-out waiting line. Fixed arrival times `[0, 1, 2, 13, 14]` and service durations `[3, 2, 1, 1, 1]`, measured in seconds, make each run deterministic without an RNG. `Reverse<Event>` turns Rust's max-heap into a min-heap. The event priority processes departures before arrivals at the same timestamp; customer IDs break any remaining ties.

The five customers wait `[0, 2, 3, 0, 0]` seconds before service, so the reported average is `5 / 5 = 1.0` seconds. All five are served, including the customers who arrive after the server becomes idle. The simulation drains the event schedule, including the final departure, before reporting its result.

```rust,editable
{{#include ../../../crates/cats/simulation/examples/simulation/discrete_event.rs:example}}
```

## Agent-Based Models {#agent-based}

[![krABMaga][c~krabmaga~docs~badge]][c~krabmaga~docs]{{hi:Agent-based modeling}}

[`krabmaga`][c~krabmaga~docs] is a Rust engine for agent-based models, with parallel execution and optional visualization. Its [release page][krabmaga~releases] currently lists 0.6.1; check recent activity, compatibility, and feature requirements before adopting it.

## Numerical ODE Solving {#ode-solving}

[![ode_solvers][c~ode_solvers~docs~badge]][c~ode_solvers~docs]{{hi:Ordinary differential equations}}

[`ode_solvers`][c~ode_solvers~docs] provides fixed-step RK4 and adaptive DOPRI5/DOP853 methods. Use it for explicit ODEs when a direct numerical solution is sufficient; [`diffsol`][c~diffsol~docs] offers additional stiff and implicit solvers.

The example integrates `dy/dt = -y`, then compares the numerical value at `t = 1` with the analytic solution `exp(-1)`.

```rust,editable
{{#include ../../../crates/cats/simulation/examples/simulation/ode_solver.rs:example}}
```

## Monte Carlo Simulation {#monte-carlo}

Monte Carlo methods use repeated random samples to estimate a distribution or expected value. Start with [[randomness | Random Numbers]] for Rust's `rand` ecosystem.

## Physics Simulation {#physics-simulation}

For rigid-body simulation and physics engines, see the [Physics Engines overview](index.md#physics-engines).

## Related Topics {#related-topics .skip}

- [[science_robotics | Robotics]].
- [[game-development | Game Development]].
- [[visualization | Visualization]].
- [Physics Engines](index.md#physics-engines).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
