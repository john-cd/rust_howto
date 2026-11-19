<!-- TODO Reviewed ??-??-2026. -->

# Simulation

[![cat~simulation][cat~simulation~badge]][cat~simulation]{{hi:Simulation}}

Choose a simulation method based on what changes in your model. A queue advances at arrivals and departures, an ODE solver approximates continuous change, and a Monte Carlo model estimates an outcome by repeated sampling.

| Domain | Rust Crate(s) | Notes |
| --- | --- | --- |
| Physics | [`rapier`][c~rapier~docs]↗{{hi:rapier}} | Rigid-body physics; see [Physics Engines](#physics-engines). [`bevy`][c~bevy~docs]↗{{hi:bevy}} integrates it via `bevy_rapier` when visualization matters. |
| Discrete-event simulation | [`nexosim`][c~nexosim~docs]↗{{hi:nexosim}}, [`desim`][c~desim~docs]↗{{hi:desim}} | `nexosim` models interacting components on a simulation clock. `desim` provides a smaller discrete-event toolkit. |
| Agent-based models | [`krabmaga`][c~krabmaga~docs]↗{{hi:krabmaga}} | Framework for agent-based simulation with optional visualization and parallel runs. |
| Numerical ODE solving | [`ode_solvers`][c~ode_solvers~docs]↗{{hi:ode_solvers}}, [`diffsol`][c~diffsol~docs]↗{{hi:diffsol}} | Runge-Kutta solvers; `diffsol` adds stiff (implicit) solvers. |
| Monte Carlo | [`rand`][c~rand~docs]↗{{hi:rand}} | See random number generation in Related Topics. |

The examples use small models with predictable results. When adapting them, choose units and solver tolerances appropriate to the system, record random seeds, and compare the output against measurements or a reference model.

## Physics Engines

{{#include physics_engines.incl.md}}

## Other Code Examples

{{#include simulation.incl.md}}

## Related Topics

- [[visualization | Visualization]]: [`plotters`][c~plotters~docs]↗{{hi:plotters}} can display simulation results.
- [[game_engines | Game Engines]]: [`bevy`][c~bevy~docs]↗{{hi:bevy}} and graphics APIs such as [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}} can support visual simulations.
- [[linear_algebra | Linear Algebra]]: [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}}, [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}}, and [`statrs`][c~statrs~docs]↗{{hi:statrs}} support numerical work.
- [[concurrency | Concurrency]]: [`rayon`][c~rayon~docs]↗{{hi:rayon}}, [`tokio`][c~tokio~docs]↗{{hi:tokio}}, and [`mpi-rs`][c~mpi~docs]↗{{hi:mpi-rs}} support different parallel and distributed workloads.
- [[randomness | Random Numbers]]: [`rand`][c~rand~docs]↗{{hi:rand}} supplies random-number generation.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
