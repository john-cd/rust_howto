# Simulation

[![cat~simulation][cat~simulation~badge]][cat~simulation]{{hi:Simulation}}

Crates used to model or construct models for some activity, e.g. to simulate a networking protocol.

The Rust ecosystem offers several options for building simulations, ranging from game engines that can be adapted for simulation to specialized crates for specific simulation domains.

- Some simulation domains have more mature crate options than others. Areas like DES, ABM, CFD, FEA, and MD are still developing in pure Rust.
- Game engines can be adapted for simulations with visual components.
- Consider adapting game engines like [`bevy`][c~bevy~docs]↗{{hi:bevy}} for simulations, especially if visualization is important.
- Integrate with existing simulation libraries (often written in other languages) using FFI. See [[development-tools_ffi | Development Tools: FFI]] and [[external-ffi-bindings | External FFI Bindings]].

| Topic | Rust Crate(s) | Notes |
|---|---|---|
| Physics Engines | `rapier`, `bevy_rapier` (Bevy integration) | `rapier` is a physics engine focused on performance. `bevy_rapier` integrates `rapier` with the [`bevy`][c~bevy~docs]↗{{hi:bevy}} game engine. `nphysics2d` is another physics engine. |
| Discrete Event Simulation (DES) | | This area is still developing in pure Rust. General-purpose crates might be used to implement DES logic. |
| Agent-Based Modeling (ABM) | | Similar to DES, ABM libraries are emerging. General-purpose crates and custom logic are often used. |
| Computational Fluid Dynamics (CFD) |  | CFD often involves complex numerical computations. Rust's numerical libraries can be used, but dedicated CFD crates are less common. |
| Finite Element Analysis (FEA) | | Similar to CFD, FEA relies on numerical computation and is an area where development is ongoing. |
| Molecular Dynamics (MD) | | MD simulations also rely heavily on numerical computation. |

## Physics Engines

{{#include physics_engine.incl.md}}

## Other Code Examples

{{#include simulation.incl.md}}

## Related Topics

| Topic | Rust Crate(s) | Notes |
|---|---|---|
| Data [[visualization | Visualization]] | [`plotters`][c~plotters~docs]↗{{hi:plotters}} |  |
| [[game_engines | Game Engines]] (Adaptable for Simulation) | [`bevy`][c~bevy~docs]↗{{hi:bevy}}, [`amethyst`][c~amethyst~docs]↗{{hi:amethyst}}, [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}} (lower-level graphics) | Game engines provide a foundation for simulations, especially those with visual components. [`bevy`][c~bevy~docs]↗{{hi:bevy}} is a popular data-driven game engine. [`amethyst`][c~amethyst~docs]↗{{hi:amethyst}} is another option. |
| Numerical Computation & [[linear_algebra | Linear Algebra]] | [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}}, [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}}, [`statrs`][c~statrs~docs]↗{{hi:statrs}} | |
| Parallel & Distributed Computing, [[concurrency | Concurrency]] | Often uses crates like [`rayon`][c~rayon~docs]↗{{hi:rayon}}, [`tokio`][c~tokio~docs]↗{{hi:tokio}}. [`mpi-rs`][c~mpi~docs]↗{{hi:mpi-rs}} provides bindings for MPI. | Rust's concurrency features can be used to parallelize simulations. |
| [[randomness | Random]] Number Generation | [`rand`][c~rand~docs]↗{{hi:rand}} | |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/961)
</div>
