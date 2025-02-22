# Simulation

[![cat-simulation][cat-simulation-badge]][cat-simulation]{{hi:Simulation}}

Crates used to model or construct models for some activity, e.g. to.simulate a networking protocol.

{{#include simulation.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 write](https://github.com/john-cd/rust_howto/issues/961)

## Rust Crates for Simulation

The Rust ecosystem offers several options for building simulations, ranging from game engines that can be adapted for simulation to specialized crates for specific simulation domains.

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| Game Engines (Adaptable for Simulation) | `bevy`, [`amethyst`][c-amethyst]⮳{{hi:amethyst}}, `wgpu` (lower-level graphics) | Game engines provide a foundation for simulations, especially those with visual components. [`bevy`][c-bevy]⮳{{hi:bevy}} is a popular data-driven game engine. [`amethyst`][c-amethyst]⮳{{hi:amethyst}} is another option. [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is a lower-level crate useful if you need fine-grained control over rendering for your simulation. |
| Physics Engines | `rapier`, `bevy_rapier` (Bevy integration), [`nphysics`][c-nphysics]⮳{{hi:nphysics}} | `rapier` is a physics engine focused on performance. `bevy_rapier` integrates `rapier` with the [`bevy`][c-bevy]⮳{{hi:bevy}} game engine. [`nphysics`][c-nphysics]⮳{{hi:nphysics}} is another physics engine. |
| Discrete Event Simulation (DES) | (Developing area) | This area is still developing in pure Rust. General-purpose crates might be used to implement DES logic. |
| Agent-Based Modeling (ABM) | (Developing area) | Similar to DES, ABM libraries are emerging. General-purpose crates and custom logic are often used. |
| Computational Fluid Dynamics (CFD) | (Developing area, often uses numerical libraries) | CFD often involves complex numerical computations. Rust's numerical libraries can be used, but dedicated CFD crates are less common. |
| Finite Element Analysis (FEA) | (Developing area, often uses numerical libraries) | Similar to CFD, FEA relies on numerical computation and is an area where development is ongoing. |
| Molecular Dynamics (MD) | (Developing area, often uses numerical libraries) | MD simulations also rely heavily on numerical computation. |
| Parallel & Distributed Simulation | (Often uses concurrency crates like [`rayon`][c-rayon]⮳{{hi:rayon}}, [`tokio`][c-tokio]⮳{{hi:tokio}}, [`mpi-rs`][c-mpi]⮳{{hi:mpi-rs}}) | Rust's concurrency features can be used to parallelize simulations. [`mpi-rs`][c-mpi]⮳{{hi:mpi-rs}} provides bindings for MPI for distributed simulations. |
| Numerical Computation (Foundation for many simulations) | [`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}}, [`ndarray`][c-ndarray]⮳{{hi:ndarray}}, [`statrs`][c-statrs]⮳{{hi:statrs}} | These crates are essential for numerical computations used in various simulations. |
| Data Visualization | [`plotters`][c-plotters]⮳{{hi:plotters}}, [`iced`][c-iced]⮳{{hi:iced}}, [`egui`][c-egui]⮳{{hi:egui}} | Essential for visualizing simulation results. |
| Random Number Generation | [`rand`][c-rand]⮳{{hi:rand}} | Crucial for simulations involving randomness. |

## Key Considerations

- Maturity: Some simulation domains have more mature crate options than others. Areas like DES, ABM, CFD, FEA, and MD are still developing in pure Rust.
- Performance: Rust's performance is a significant advantage for computationally intensive simulations.
- Graphics & Visualization: Game engines can be adapted for simulations with visual components.
- Numerical Computation: Rust's numerical libraries are fundamental for many types of simulations.
- Concurrency: Rust's concurrency features are beneficial for parallelizing and distributing simulations.

## Strategies for Development

- Game Engines: Adapt game engines like [`bevy`][c-bevy]⮳{{hi:bevy}} for simulations, especially if visualization is important.
- Numerical Libraries: Build simulations using Rust's numerical computation crates.
- FFI: Integrate with existing simulation libraries (often written in other languages) using FFI.
- Community Building: Contribute to the development of new Rust crates for specific simulation domains.

</div>
