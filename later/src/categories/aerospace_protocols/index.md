<!-- TODO Reviewed ??-??-2026 -->

# Aerospace Protocols

[![cat~aerospace::protocols][cat~aerospace::protocols~badge]][cat~aerospace::protocols]{{hi:Aerospace protocols}}

{{#include aerospace_protocols.incl.md}}

This guide surveys protocol choices for aircraft, uncrewed vehicles, and spacecraft. Use the linked protocol or device documentation for implementation details; this page focuses on crate support and hardware boundaries.

## Common Aerospace Protocols

### MAVLink (Micro Air Vehicle Link)

MAVLink is a widely used protocol for communication between drones and other unmanned vehicles and ground control stations. It is message-based and defines standard messages for flight control, telemetry, and sensor data.

The [`mavlink`][c~mavlink~docs] crate provides message encoding and decoding. The selected dialect and autopilot determine which messages and commands are supported.

### CAN (Controller Area Network)

CAN is a robust serial communication protocol used between electronic components such as sensors, actuators, and controllers.

For Linux systems, [`socketcan`][c~socketcan~docs] exposes SocketCAN interfaces and frames, often through USB-CAN adapters. Embedded targets can use the [`embedded-can`][c~embedded-can~docs] traits with a compatible hardware driver; [`embedded-hal`][c~embedded-hal~docs] provides the broader hardware abstraction layer used by many such drivers. These crates cover different layers, so check which one the device and driver need.

### ARINC 429

ARINC 429 is a widely used standard for data transfer in commercial aircraft avionics. It is a unidirectional, broadcast-oriented protocol.

The [`arinc429`][c~arinc429~docs] crate is a small Rust implementation to evaluate for word encoding and decoding. Direct ecosystem support remains limited; before using a crate in avionics, check its current maintenance, test coverage, and conformance against the licensed standard and target hardware. Existing C or C++ libraries may require [[development-tools_ffi | FFI]].

### MIL-STD-1553

MIL-STD-1553 is a serial data bus used in reliable, fault-tolerant military and aerospace systems.

The [`mil_std_1553b`][c~mil_std_1553b~docs] crate provides Rust support for MIL-STD-1553B protocol data, though mission-ready support still depends on compatible bus-controller or remote-terminal hardware and a validated driver. Confirm the crate's current status and mission requirements before adoption; FFI or a custom implementation may be needed for a particular device.

### SpaceWire

SpaceWire is a high-speed serial data bus commonly used in spacecraft and other space applications.

I found no maintained SpaceWire crate to recommend. A hardware vendor's SDK or a project-specific implementation may be needed; validate link initialization, routing, and error handling against the device and applicable standard. FFI may be required when integrating an existing vendor library.

### CCSDS (Consultative Committee for Space Data Systems)

CCSDS is a set of standards for space data systems, including packet telemetry and telecommand protocols. The [`spacepackets`][c~spacepackets~docs] crate implements CCSDS packets and ECSS PUS packet formats. For packet serialization examples and the [`satrs`][c~satrs~docs] framework, see the [[aerospace_space-protocols | Space Protocols]] category.

## Related Topics

- [[aerospace | Aerospace]].
- [[aerospace_drones | Drones]].
- [[aerospace_simulation | Aerospace Simulation]].
- [[aerospace_space-protocols | Space Protocols]].
- [[aerospace_unmanned-aerial-vehicles | Unmanned Aerial Vehicles]].
- [[hardware-support | Hardware Support]].
- [[simulation | Simulation]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
