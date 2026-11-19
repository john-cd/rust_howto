<!-- TODO Reviewed ??-??-2026. -->

# Aerospace

{{#include aerospace.incl.md}}

## Sample an Archived Earth Orbit {#aero}

A two-line element set (TLE) records a satellite's mean orbital elements and their epoch. This example parses an archived ISS element set with [sgp4][c~sgp4~docs], initializes the propagator once, and samples roughly one orbit in ten-minute steps.

Run from the repository root.

```console
just cargo run --manifest-path later/crates/Cargo.toml --locked -p aerospace --example aerospace
```

The first line corresponds to the TLE epoch. SGP4 returns positions in kilometers and velocities in kilometers per second in the True Equator, Mean Equinox (TEME) frame. Keep the timestamp and frame with the coordinates when saving or plotting them. A latitude and longitude on a rotating Earth require a frame conversion. For a single prediction at a chosen offset, see [[aerospace_simulation | Aerospace Simulation]].

```rust,editable
{{#include ../../../crates/cats/aerospace/examples/aerospace/aero.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[aerospace_protocols | Aerospace Protocols]].
- [[aerospace_simulation | Aerospace Simulation]].
- [[aerospace_drones | Drones]].
- [[simulation | Simulation]].
- [[aerospace_space-protocols | Space Protocols]].
- [[uavs | UAVs]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
