<!-- TODO Reviewed ??-??-2026 -->

# Aerospace

{{#include aerospace.incl.md}}

## Aerospace {#aero}

This example samples a historical ISS orbit at ten-minute intervals. Positions are in kilometers in the TEME frame; use current element sets for operational predictions. For a single prediction at a chosen time, see [[aerospace_simulation | Aerospace Simulation]].

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
