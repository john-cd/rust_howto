<!-- TODO Reviewed ??-??-2026 -->

# Aerospace - Space Protocols

{{#include space_protocols.incl.md}}

## Serialize and parse a CCSDS space packet {#ccsds-space-packet}

The CCSDS Space Packet Protocol places an application process identifier (APID), sequence flags and count, and data length in a primary header. This recipe serializes a packet with a short payload, then parses the header and checks those fields.

```rust,editable
{{#include ../../../crates/cats/aerospace_space_protocols/examples/space_protocols/space_protocols.rs:ccsds-space-packet}}
```

## Create a PUS telecommand {#pus-telecommand}

The ECSS Packet Utilization Standard (PUS-C) defines packet formats and services layered on CCSDS space packets. This recipe creates a service 17, subservice 1 connection-test telecommand and parses the serialized packet.

```rust,editable
{{#include ../../../crates/cats/aerospace_space_protocols/examples/space_protocols/space_protocols.rs:pus-telecommand}}
```

The [`spacepackets`][c~spacepackets~docs] crate implements CCSDS packets and PUS-C telecommands and telemetry. The [`satrs`][c~satrs~docs] crate is a framework for on-board software with event management, PUS support, and telemetry and telecommand routing; its latest published version is 0.2.1, so verify current maintenance and mission fit before adoption.

## COP-1

The CCSDS Communications Operation Procedure 1 (COP-1) provides reliable telecommand transfer over a space data link. It complements packet construction: a mission also needs a compatible link-layer implementation and verification strategy.

## Mission control integration

SCOS-2000 is an ESA spacecraft control and operations system. Integration is specific to the mission's ground-system interfaces and supported libraries; the packet examples here do not implement that system.

## Related Topics {#related-topics .skip}

- [[aerospace | Aerospace]].
- [[aerospace_protocols | Aerospace Protocols]].
- [[parsing | Parsing]].
- [[encoding | Encoding]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
