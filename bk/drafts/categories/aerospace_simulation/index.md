# Aerospace Simulation

[![cat-aerospace::simulation][cat-aerospace::simulation-badge]][cat-aerospace::simulation]{{hi:Aerospace simulations}}

{{#include aerospace_simulation.incl.md}}

- Flight Dynamics :Simulating the motion of aircraft or spacecraft, including forces (lift, drag, thrust, gravity), equations of motion, and aerodynamic models.
- Orbital Mechanics: Simulating the motion of satellites or other objects in orbit, including orbital parameters, Kepler's laws, and perturbation effects.
- Control Systems: Simulating the behavior of control systems, such as PID controllers, attitude control systems, and guidance algorithms.
- Environment Modeling: Creating models of the environment, including terrain, atmosphere, and other objects (e.g., other aircraft, obstacles).
- Sensor Simulation: Simulating the behavior of sensors, such as IMUs, GPS receivers, and cameras.
- Multi-body Dynamics: Simulating the interaction of multiple bodies.

## Physics Engines

There are few mature physics engines in pure Rust. Many robust physics engines are written in C/C++.

- `rapier`: A 2D and 3D physics engine written in Rust. It might be suitable for some aerospace simulations, especially if you don't need highly specialized aerospace physics.
- `bevy_rapier`: Integration between `rapier` and the `bevy` game engine.

See also:

- [[game-development | Game Development]].
- [[game-engines | Game Engines]].
- [[game_development | Game Development]].
- [[game_engines | Game Engines]].
- [[simulation | Simulation]].

## FFI (Foreign Function Interface)

For more advanced aerospace simulations, you'll likely need to use FFI to interact with existing C/C++ physics engines like:

- `Bullet`: A popular open-source physics engine.
- `ODE` (Open Dynamics Engine): Another open-source option.
- `Simbody`: A high-performance library for multibody dynamics (often used in robotics and aerospace).

See:

- [[development-tools_ffi | FFI]].
- [[external-ffi-bindings | External FFI Bindings]].
- [[generate_ffi_bindings | Generate FFI Bindings]].

## Linear Algebra

See [[mathematics | Mathematics]] and [[linear_algebra | Linear Algebra]].

Consider using:

- `nalgebra`: Excellent for vector, matrix, and quaternion operations, which are fundamental to aerospace simulations.
- `ndarray`: Another good option for numerical array operations.

## Visualization

See [[visualization | Visualization]].

Consider using:

- [`bevy`][c-bevy]⮳{{hi:bevy}}, which is a data-driven game engine suitable for 3D visualization.
- [`plotters`][c-plotters]⮳{{hi:plotters}} for creating plots and charts of simulation data.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[aerospace_simulation: write](https://github.com/john-cd/rust_howto/issues/199)
</div>
