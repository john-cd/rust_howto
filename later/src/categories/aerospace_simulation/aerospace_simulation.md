<!-- TODO Reviewed ??-??-2026. -->

# Aerospace Simulation

{{#include aerospace_simulation.incl.md}}

## Simulate a Rocket Launch {#rocket-launch-simulation}

This from-scratch rocket example demonstrates a simple time-stepped model, with mass, thrust, drag, and a ground boundary. It is illustrative rather than a flight-certified simulation. The following recipes use libraries for orbital propagation and numerical integration.

```rust,editable
{{#include ../../../crates/cats/aerospace_simulation/examples/aerospace_simulation/aero_simulation.rs:example}}
```

## Propagate a TLE with SGP4 {#sgp4-tle-propagation}

Run the simulation examples locally from the repository root. They use built-in inputs.

```console
just cargo run --manifest-path later/crates/Cargo.toml --locked -p aerospace_simulation --example aerospace_simulation
```

SGP4 propagates a satellite's two-line element (TLE) set from its epoch. This fixed example evaluates the ISS TLE sixty minutes after epoch; the returned position is in kilometers in the TEME frame. TLEs and predictions are time-sensitive, so use current orbital elements for operational work.

```rust,editable
{{#include ../../../crates/cats/aerospace_simulation/examples/aerospace_simulation/aero_simulation.rs:sgp4-tle-propagation}}
```

The [`sgp4`][c~sgp4~docs] crate provides the propagator. For more precise time-scale and date-time handling, consider [`hifitime`][c~hifitime~docs], which supports time scales such as UTC, TAI, and TDB. The [`uom`][c~uom~docs] crate can represent physical quantities with units and dimensions. Use hifitime when observation timestamps and orbital epochs come from different time scales. Use uom when a model combines inputs such as meters and kilometers. This example passes minutes to SGP4 and keeps the units explicit at the call site.

## Solve equations of motion with RK4 {#rk4-ballistic-trajectory}

An ordinary differential equation solver can integrate coupled equations of motion. Here, [`ode_solvers`][c~ode_solvers~docs] runs a fixed-step fourth-order Runge-Kutta (RK4) method for a vertical trajectory under constant gravity. The state stores altitude in meters and velocity in meters per second.

```rust,editable
{{#include ../../../crates/cats/aerospace_simulation/examples/aerospace_simulation/aero_simulation.rs:rk4-ballistic-trajectory}}
```

## Related Topics {#related-topics .skip}

- [[aerospace | Aerospace]].
- [[simulation | Simulation]].
- [[mathematics | Mathematics]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
