<!-- TODO Reviewed ??-??-2026. -->

# Aerospace Simulation

[![cat~aerospace::simulation][cat~aerospace::simulation~badge]][cat~aerospace::simulation]{{hi:Aerospace simulations}}

{{#include aerospace_simulation.incl.md}}

Choose the model according to the data you need. An orbital propagator predicts position and velocity at a specified time. An ordinary differential equation integrator advances a user-defined state through small time steps. Sensor models can add sample timing, measurement noise, and missing readings to simulated data.

Start with the [SGP4 recipe](aerospace_simulation.md#sgp4-tle-propagation). Keep the initial conditions, units, time scale, and coordinate frame with the simulation output so someone else can reproduce it.

## Physics Engines

A physics engine is useful for collision or visual simulations. Orbital predictions use a propagator with a gravity model, a time scale, and a coordinate frame appropriate for the data.

- [`rapier`][rapier.rs~website]↗{{hi:rapier}}: A 2D and 3D physics engine written in Rust. It might be suitable for some aerospace simulations, especially if you don't need highly specialized aerospace physics.
- `bevy_rapier`: Integration between [`rapier`][rapier.rs~website]↗{{hi:rapier}} and the [`bevy`][c~bevy~docs]↗{{hi:bevy}} game engine.

See also:

- [Game Development][rust-howto~game-development].
- [Game Engines][rust-howto~game-engines].
- [[simulation | Simulation]].

## FFI (Foreign Function Interface)

For more advanced aerospace simulations, you'll likely need to use FFI to interact with existing C/C++ physics engines like:

- [`Bullet`][wikipedia~bullet]↗{{hi:Bullet}}: A popular open-source physics engine.
- [`ODE`][ode~website]↗{{hi:ODE}} (Open Dynamics Engine): Another open-source option.
- [`Simbody`][simbody~repo]↗{{hi:Simbody}}: A high-performance library for multibody dynamics (often used in robotics and aerospace).

See:

- [[development-tools_ffi | FFI]].
- [[external-ffi-bindings | External FFI Bindings]].
- [[generate_ffi_bindings | Generate FFI Bindings]].

## Linear Algebra

See [[mathematics | Mathematics]] and [[linear_algebra | Linear Algebra]].

Consider using:

- [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}}: Excellent for vector, matrix, and quaternion operations, which are fundamental to aerospace simulations.
- [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}}: Another good option for numerical array operations.

## Visualization

See [[visualization | Visualization]].

Consider using:

- [`bevy`][c~bevy~docs]↗{{hi:bevy}}, which is a data-driven game engine suitable for 3D visualization.
- [`plotters`][c~plotters~docs]↗{{hi:plotters}} for creating plots and charts of simulation data.

## Related Topics

- [[aerospace | Aerospace]].
- [[simulation | Simulation]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
