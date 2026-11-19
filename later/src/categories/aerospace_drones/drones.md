<!-- TODO Reviewed ??-??-2026. -->

# Aerospace - Drones

{{#include drones.incl.md}}

## Test a Drone Plan with a Recorder {#drones}

Use a small interface between plan validation and hardware I/O. The `Flyer` trait gives the tests an in-memory recorder. They check that a valid plan issues takeoff before landing, and that an invalid plan issues zero commands.

```console
just cargo test --manifest-path later/crates/Cargo.toml --locked -p aerospace_drones --example drones
```

The live entry point uses [tello][c~tello~docs] with a compatible DJI Tello and sends flight commands. The live test is ignored by default. For a receive-only workflow and a reproducible telemetry fixture, see [Tello flight telemetry](../aerospace_unmanned-aerial-vehicles/uavs.md#uavs).

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
