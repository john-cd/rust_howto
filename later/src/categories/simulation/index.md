<!-- TODO Reviewed ??-??-2026 -->

# Simulation

[![cat~simulation][cat~simulation~badge]][cat~simulation]{{hi:Simulation}}

Crates used to model or construct models for some activity, e.g. to simulate a networking protocol.

Scope: this category covers software that models a system over time or space. The domains below are the ones with usable Rust crates today; others (CFD, FEA, molecular dynamics) rely on general numerical crates, bindings through [[development-tools_ffi | FFI]], or custom code.

| Domain                    | Rust Crate(s)                                                                                      | Notes                                                                                                                                                      |
| ------------------------- | -------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Physics                   | [`rapier`][c~rapier~docs]↗{{hi:rapier}}                                                            | Rigid-body physics; see [Physics Engines](#physics-engines). [`bevy`][c~bevy~docs]↗{{hi:bevy}} integrates it via `bevy_rapier` when visualization matters. |
| Discrete-event simulation | [`nexosim`][c~nexosim~docs]↗{{hi:nexosim}}, [`desim`][c~desim~docs]↗{{hi:desim}}                   | `nexosim` is the actively developed, general-purpose option; `desim` is small and appears unmaintained. Check release dates before adopting either.        |
| Agent-based models        | [`krabmaga`][c~krabmaga~docs]↗{{hi:krabmaga}}                                                      | Framework for agent-based simulation with optional visualization and parallel runs.                                                                        |
| Numerical ODE solving     | [`ode_solvers`][c~ode_solvers~docs]↗{{hi:ode_solvers}}, [`diffsol`][c~diffsol~docs]↗{{hi:diffsol}} | Runge-Kutta solvers; `diffsol` adds stiff (implicit) solvers.                                                                                              |
| Monte Carlo               | [`rand`][c~rand~docs]↗{{hi:rand}}                                                                  | See random number generation in Related Topics.                                                                                                            |

Follow-up pages worth writing (discrete-event, agent-based models, ODE solving) are tracked in the epic [#1994](https://github.com/john-cd/rust_howto/issues/1994).

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
