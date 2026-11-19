<!-- TODO Reviewed ??-??-2026. -->

# Aerospace - Space Protocols

{{#include space_protocols.incl.md}}

## Serialize and Parse a CCSDS Space Packet {#ccsds-space-packet}

The CCSDS Space Packet Protocol starts with a six-byte primary header. The application process identifier (APID) identifies a data source in the mission's packet map. The sequence fields describe packet ordering and segmentation. The encoded length is one less than the number of bytes that follow the primary header.

```console
just cargo run --manifest-path later/crates/Cargo.toml --locked -p aerospace_space_protocols --example space_protocols -- ccsds
```

This example writes an eight-byte telemetry packet and extracts its two data bytes (`CA FE`). Use the total length from the header to delimit the packet when several packets share a capture buffer. A short buffer is rejected. A real decoder also needs the mission's APID map to interpret secondary headers and data fields.

```rust,editable
{{#include ../../../crates/cats/aerospace_space_protocols/examples/space_protocols/space_protocols.rs:ccsds-space-packet}}
```

## Create a PUS telecommand {#pus-telecommand}

The ECSS Packet Utilization Standard (PUS-C) defines packet formats and services layered on CCSDS space packets. This recipe creates a service 17, subservice 1 connection-test telecommand and parses the serialized packet.

```rust,editable
{{#include ../../../crates/cats/aerospace_space_protocols/examples/space_protocols/pus_telecommand.rs:pus-telecommand}}
```

## Create PUS telemetry {#pus-telemetry}

This response uses service 17, subservice 2 and a fixed timestamp, then decodes the serialized telemetry packet.

```rust,editable
{{#include ../../../crates/cats/aerospace_space_protocols/examples/space_protocols/pus_telemetry.rs:pus-telemetry}}
```

The [`spacepackets`][c~spacepackets~docs] crate implements CCSDS packets and PUS-C telecommands and telemetry. The [`satrs`][c~satrs~docs] crate is a framework for on-board software with event management, PUS support, and telemetry and telecommand routing.

## COP-1

The CCSDS Communications Operation Procedure 1 (COP-1) provides reliable telecommand transfer over a space data link. It complements packet construction: a mission also needs a compatible link-layer implementation and verification strategy.

## Mission control integration

SCOS-2000 is an ESA spacecraft control and operations system. Integration depends on mission interfaces and supported libraries; existing native libraries may require [[development-tools_ffi | FFI]]. Start by mapping the mission packet definitions and timestamp representation to the ground system interface.

## Related Topics {#related-topics .skip}

- [[aerospace | Aerospace]].
- [[aerospace_protocols | Aerospace Protocols]].
- [[parsing | Parsing]].
- [[encoding | Encoding]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
