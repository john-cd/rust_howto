<!-- TODO Reviewed ??-??-2026 -->

# Unmanned Aerial Vehicles

{{#include uavs.incl.md}}

## Unmanned Aerial Vehicles {#uavs}

Here is an example of reading telemetry from a DJI Tello drone using the `tello` crate (last released in 2021; see the [drones](../aerospace_drones/drones.md) chapter for the caveat). Decoding a message into text is a pure function, so the tests decode a synthetic flight-data packet without a drone; the test that talks to a real drone is ignored. To send commands instead, see the drones chapter.

```rust,editable
{{#include ../../../crates/cats/aerospace_unmanned_aerial_vehicles/examples/uavs/uavs.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[aerospace | Aerospace]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
