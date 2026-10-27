<!-- TODO Reviewed ??-??-2026 -->

# Aerospace Simulation

{{#include simulation.incl.md}}

## Simulate a Rocket Launch {#rocket-launch-simulation}

The existing rocket example demonstrates a simple time-stepped model. The following recipes add an orbital propagation example and a fixed-step numerical solver for an ordinary differential equation.

```rust,editable
{{#include ../../../crates/cats/aerospace_simulation/examples/aerospace_simulation/aero_simulation.rs:example}}
```

## Propagate a TLE with SGP4 {#sgp4-tle-propagation}

SGP4 propagates a satellite's two-line element (TLE) set from its epoch. This fixed example evaluates the ISS TLE sixty minutes after epoch; the returned position is in kilometers in the TEME frame. TLEs and predictions are time-sensitive, so use current orbital elements for operational work.

```rust,editable
{{#include ../../../crates/cats/aerospace_simulation/examples/aerospace_simulation/aero_simulation.rs:sgp4-tle-propagation}}
```

The [`sgp4`][c~sgp4~docs] crate provides the propagator. For more precise time-scale and date-time handling, consider [`hifitime`][c~hifitime~docs], which supports time scales such as UTC, TAI, and TDB. The [`uom`][c~uom~docs] crate can represent physical quantities with units and dimensions. Both crates continue to publish updated releases; check their current documentation and features for the target application.

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
