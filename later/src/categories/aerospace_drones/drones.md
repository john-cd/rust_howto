<!-- TODO Reviewed ??-??-2026 -->

# Aerospace - Drones

{{#include drones.incl.md}}

## Drones {#drones}

Drones (UAVs) can be programmed and controlled via various protocols such as MAVLink or vendor-specific SDKs. The `tello` crate provides a straightforward interface to control DJI Tello drones. The example below sends commands: it checks a flight plan (a take-off, a short hover, a landing, with a hover limit) and then sends it to the drone. The plan logic is tested without a drone through a small `Flyer` trait; the test that flies a real drone is ignored. The `tello` crate was last released in 2021, so treat it as unmaintained: check that it still works with your drone firmware, or use the drone's text-based SDK over UDP directly.

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
