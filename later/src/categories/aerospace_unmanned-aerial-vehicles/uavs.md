<!-- TODO Reviewed ??-??-2026. -->

# Unmanned Aerial Vehicles

{{#include uavs.incl.md}}

## Decode Tello Flight Telemetry {#uavs}

This example uses [tello][c~tello~docs] to decode a synthetic flight-data packet and prints `battery 87%, height 12 dm`.

```console
just cargo run --manifest-path later/crates/Cargo.toml --locked -p aerospace_unmanned_aerial_vehicles --example uavs
```

With access to a compatible Tello on its Wi-Fi network, append `-- --live` to open a telemetry connection and poll once. The poll can return a connection acknowledgement or no data if no datagram has arrived. For a dashboard, poll continuously and attach a receive time to each sample so you can mark stale readings. The offline fixture uses placeholder CRC bytes because this decoder accepts them. Validate framing and integrity separately before relying on arbitrary datagrams.

```rust,editable
{{#include ../../../crates/cats/aerospace_unmanned_aerial_vehicles/examples/uavs/uavs.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[aerospace | Aerospace]].
- [Drones](../aerospace_drones/drones.md).
- [MAVLink heartbeats](../aerospace_protocols/aerospace_protocols.md#aerospace_protocols).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
