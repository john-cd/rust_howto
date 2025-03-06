# Simulation

[![cat-simulation][cat-simulation-badge]][cat-simulation]{{hi:Simulation}}

Crates used to model or construct models for some activity, e.g. to simulate a networking protocol.

The Rust ecosystem offers several options for building simulations, ranging from game engines that can be adapted for simulation to specialized crates for specific simulation domains.

- Some simulation domains have more mature crate options than others. Areas like DES, ABM, CFD, FEA, and MD are still developing in pure Rust.
- Game engines can be adapted for simulations with visual components.
- Consider adapting game engines like [`bevy`][c-bevy]⮳{{hi:bevy}} for simulations, especially if visualization is important.
- Integrate with existing simulation libraries (often written in other languages) using FFI. See [[development-tools_ffi | Development Tools: FFI]] and [[external-ffi-bindings | External FFI Bindings]].

| Topic | Rust Crate(s) | Notes |
|---|---|---|
| Physics Engines | `rapier`, `bevy_rapier` (Bevy integration), [`nphysics`][c-nphysics]⮳{{hi:nphysics}} | `rapier` is a physics engine focused on performance. `bevy_rapier` integrates `rapier` with the [`bevy`][c-bevy]⮳{{hi:bevy}} game engine. [`nphysics`][c-nphysics]⮳{{hi:nphysics}} is another physics engine. |
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
| Data [[visualization | Visualization]] | [`plotters`][c-plotters]⮳{{hi:plotters}} |  |
| [[game_engines | Game Engines]] (Adaptable for Simulation) | [`bevy`][c-bevy]⮳{{hi:bevy}}, [`amethyst`][c-amethyst]⮳{{hi:amethyst}}, [`wgpu`][c-wgpu]⮳{{hi:wgpu}} (lower-level graphics) | Game engines provide a foundation for simulations, especially those with visual components. [`bevy`][c-bevy]⮳{{hi:bevy}} is a popular data-driven game engine. [`amethyst`][c-amethyst]⮳{{hi:amethyst}} is another option. |
| Numerical Computation & [[linear_algebra | Linear Algebra]] | [`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}}, [`ndarray`][c-ndarray]⮳{{hi:ndarray}}, [`statrs`][c-statrs]⮳{{hi:statrs}} | |
| Parallel & Distributed Computing, [[concurrency | Concurrency]] | Often uses crates like [`rayon`][c-rayon]⮳{{hi:rayon}}, [`tokio`][c-tokio]⮳{{hi:tokio}}. [`mpi-rs`][c-mpi]⮳{{hi:mpi-rs}} provides bindings for MPI. | Rust's concurrency features can be used to parallelize simulations. |
| [[randomness | Random]] Number Generation | [`rand`][c-rand]⮳{{hi:rand}} | |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/961)
</div>
