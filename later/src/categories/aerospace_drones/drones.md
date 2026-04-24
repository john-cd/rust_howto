# Aerospace - Drones

{{#include drones.incl.md}}

## Drones {#drones}

Drones (UAVs) can be programmed and controlled via various protocols such as MAVLink or vendor-specific SDKs. The `tello` crate provides a straightforward interface to control DJI Tello drones. In the example below, we connect to a drone, command it to take off, and then land.

```rust,editable
{{#include ../../../crates/cats/aerospace_drones/examples/drones/drones1.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[aerospace | Aerospace]].
- [[aerospace_protocols | Aerospace Protocols]].
- [[embedded | Embedded Systems]].
- [[uavs | UAVs]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
